//! The nativestore is a tree based store.
//! It is the basis of the local instance

use {
	crate::{
		node::{Node,Object},
		path::{Fragment,Path},
		operror::OperationError,
		DB
	},
	std::error::Error,
	sha3::{Digest, Sha3_256},
	serde::{Serialize, Deserialize},
	rand::Rng
};


/// Nativestore node tree name
const NODE_TREE: &[u8; 17] = b"NATIVESTORE_NODES";
/// Nativestore object tree name
const OBJT_TREE: &[u8; 19] = b"NATIVESTORE_OBJECTS";
/// Nativestore slug index tree name
const SLUG_TREE: &[u8; 22] = b"NATIVESTORE_SLUG_INDEX";
/// Nativestore filetype index tree name
const FRIN_TREE: &[u8; 24] = b"NATIVESTORE_FORMAT_INDEX";
/// Nativestore garbage collection reference count tree name
const GCRC_TREE: &[u8; 22] = b"NATIVESTORE_GCRC_INDEX";

const BLOCK_SIZE: usize = 1024;

#[derive(Serialize, Deserialize, Clone)]
/// Fragment for nativestore
/// Since we know the storage we don't track it
struct NativeFragment {
	parent: u128,
	#[serde(with = "serde_bytes")]
	slug: Vec<u8>,
	#[serde(with = "serde_bytes")]
	format: Vec<u8>,
	version: u64
}

#[typetag::serde]
impl Fragment for NativeFragment {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn get_slug(&self) -> Vec<u8> {
		self.slug.clone()
	}
	fn get_handler(&self) -> Vec<u8> {
		String::from("NativeNode").into_bytes()
	}
	fn get_format(&self) -> Vec<u8> {
		self.format.clone()
	}
	fn get_version(&self) -> u64 {
		self.version.clone()
	}
}

#[derive(Serialize, Deserialize)]
pub struct NativeNodeV1 {
	/// Key of parent
	parent: Option<NativeFragment>,
	/// Internal ID
	/// First part of child keys
	id: u128,
	/// name of node
	slug: Vec<u8>,
	format: Vec<u8>,
	version: u64,
	/// Second part of child keys
	children: Vec<Box<dyn Fragment>>,
	// TODO: figure out a way to make this a bytearray
	/// Hashes of data blocks or Objects
	data: Vec<Vec<u8>>
}

#[typetag::serde]
impl Node for NativeNodeV1 {
	fn get_node(self, fragment:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		let node_tree = DB.open_tree(NODE_TREE)
			.expect("Failure opening the node tree");
		let reconstructed: NativeFragment;
		let nfrag: &NativeFragment = match fragment.as_any().downcast_ref() {
			Some(f) => f,
			None => {
				reconstructed = self.make_fragment_native(fragment);
				&reconstructed
			}
		};
		let key = bytekey_fix::serialize(nfrag)?;
		match node_tree.get(key)? {
			Some(v) => Ok(
				bincode::deserialize::<Box<dyn Node>>(&v)?
			),
			None => return Err(
				Box::new(OperationError::InternalStoreError(
					"Nativestore inconsistency (rebuild indexes)"
				))
			)
		}
	}
	fn get_nodes(self) -> Vec<Box<dyn Fragment>> {
		self.children
	}
	fn read(self, start: usize, len: usize) -> Result<Vec<u8>, Box<dyn Error>> {
		if self.data.len() > 0 {
			let mut index = start/BLOCK_SIZE;
			let mut off = start%BLOCK_SIZE;
			let obj_tree = DB.open_tree(OBJT_TREE)
				.expect("Failure opening the object tree");
			let mut out: Vec<u8> = Vec::new();
			while index <= self.data.len() && out.len() < len {
				match obj_tree.get(&self.data[index]) {
					Ok(op) => match op {
						Some(v) => out.append(&mut(v.subslice(
								off,
								if len-off > 1024 {len-off} else {1024}
							)[..]).into()),
						None => return Err(
							Box::new(OperationError::InternalStoreError(
								"Nativestore inconsistency (rebuild indexes)"
							))
						)
					},
					Err(_) => return Err(
						Box::new(
							OperationError::InternalStoreError("Sled error")
						)
					)
				}
				off = 0;
				index += 1;
			}
			out.shrink_to_fit();
			Ok(out)
		} else {
			Ok(Vec::new())
		}
	}
	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>> {
		if data.len() > 0 {
			let mut index = start/BLOCK_SIZE;
			let mut off = start%BLOCK_SIZE;
			let mut read_off: usize = 0;
			let obj_tree = DB.open_tree(OBJT_TREE)
				.expect("Failure opening the object tree");
			while index <= self.data.len() {
				let mut object = Vec::new();
				match obj_tree.get(&self.data[index]) {
					Ok(op) => match op {
						Some(v) => object = v[..].into(),
						None => return Err(
							Box::new(OperationError::InternalStoreError(
								"Nativestore inconsistency (rebuild indexes)"
							))
						)
					},
					Err(_) => return Err(
						Box::new(
							OperationError::InternalStoreError("Sled error")
						)
					)
				}
				let ilen = object.len();
				if off < ilen {
					object.splice(off..ilen, data[..read_off+(ilen-off)].iter().cloned());
					object.extend_from_slice(&data[read_off+(ilen-off)..read_off+(BLOCK_SIZE-off)]);
				} else if off == ilen {
					object.extend_from_slice(&data[read_off+(ilen-off)..read_off+(BLOCK_SIZE-off)]);
				} else {
					return Err(
						Box::new(OperationError::InternalStoreError(
							"Nativestore has not yet implemented writes past EOF"
						))
					)
				}
				let mut hasher = Sha3_256::new();
				hasher.update(object[..].iter());
				let name = hasher.finalize();
				match obj_tree.insert(name, object) {
					// !!The interior Option may need to be checked!!
					Ok(_) => {},
					Err(_) => return Err(
							Box::new(
								OperationError::InternalStoreError("Sled error")
							)
						)
				}
				off = 0;
				index += 1;
				read_off += BLOCK_SIZE-off;
			}
			Ok(data.len())
		} else {
			Ok(0)
		}
	}

	fn create_node(self, fragment:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		let nfrag: NativeFragment = match fragment.as_any().downcast_ref::<NativeFragment>() {
			Some(f) => f.clone(),
			None => {
				self.make_fragment_native(fragment)
			}
		};
		if nfrag.parent != self.id {
			return Err(
				Box::new(
					OperationError::BadMessage(
						"Passed NativeFragment with bad parent to create_node"
					)
				)
			);
		}
		let parent = NativeFragment {
			parent: match self.parent {
				Some(f) => f.parent,
				None => 0u128
			},
			slug: self.slug,
			format: self.format,
			version: self.version,
		};
		let child = NativeNodeV1::new(
			Some(parent),
			nfrag.clone()
		);
		let node_tree = DB.open_tree(NODE_TREE)
			.expect("Failure opening the node tree");
		let key = bytekey_fix::serialize(&nfrag)?;
		let value = bincode::serialize(&child)?;
		node_tree.insert(key, value)?;
		Ok(Box::new(child))
	}

	fn move_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn link_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		Err(
			Box::new(
				OperationError::NotImplemented(
					"The nativestore does not implement graph operations like link_node"
				)
			)
		)
	}

	fn unlink_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		Err(
			Box::new(
				OperationError::NotImplemented(
					"The nativestore does not implement graph operations like unlink_node"
				)
			)
		)
	}
}

impl NativeNodeV1 {
	fn make_fragment_native(&self, fragment: &dyn Fragment) -> NativeFragment {
		NativeFragment {
			parent: self.id,
			slug: fragment.get_slug(),
			format: fragment.get_format(),
			version: fragment.get_version()
		}
	}
	fn make_floating_native(fragment: &dyn Fragment) -> NativeFragment {
		NativeFragment {
			parent: 0,
			slug: fragment.get_slug(),
			format: fragment.get_format(),
			version: fragment.get_version()
		}
	}
	fn new(parent: Option<NativeFragment>, dest: NativeFragment) -> NativeNodeV1 {
		let mut rng = rand::thread_rng();
		NativeNodeV1 {
			parent: parent,
			id: rng.gen(),
			slug: dest.slug,
			format: dest.format,
			version: dest.version,
			children: Vec::new(),
			data: Vec::new()
		}
	}
	pub fn create_root(dest: &dyn Fragment) -> Result<(), Box<dyn Error>> {
		let nfrag: NativeFragment = match dest.as_any().downcast_ref::<NativeFragment>() {
			Some(f) => f.clone(),
			None => {
				NativeNodeV1::make_floating_native(dest)
			}
		};
		let child = NativeNodeV1::new(
			None,
			nfrag.clone()
		);
		let node_tree = DB.open_tree(NODE_TREE)
			.expect("Failure opening the node tree");
		let key = bytekey_fix::serialize(&nfrag)?;
		let value = bincode::serialize(&child)?;
		node_tree.insert(key, value)?;
		Ok(())
	}
	pub fn get_root(dest: &dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		let nfrag: NativeFragment = match dest.as_any().downcast_ref::<NativeFragment>() {
			Some(f) => f.clone(),
			None => {
				NativeNodeV1::make_floating_native(dest)
			}
		};
		let node_tree = DB.open_tree(NODE_TREE)
			.expect("Failure opening the node tree");
		let key = bytekey_fix::serialize(&nfrag)?;
		match node_tree.get(key)? {
			Some(v) => Ok(
				bincode::deserialize::<Box<dyn Node>>(&v)?
			),
			None => return Err(
				Box::new(OperationError::InternalStoreError(
					"Nativestore inconsistency (rebuild indexes)"
				))
			)
		}
	}
}


impl Object for Vec<Vec<u8>> {
	
}
