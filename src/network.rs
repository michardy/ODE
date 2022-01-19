//! The network store is a tree based store
//! It is the root of the ODE tree.
//! All instance Nativestores are subtrees of it

use std::sync::Arc;

use {
	crate::{
		node::{Node,Object},
		path::{Fragment,Path},
		operror::OperationError,
		nativestore::NativeNodeV1,
		DB,
		BOOT_TREE
	},
	std::error::Error,
	serde::{Serialize, Deserialize}
};

const NET_PERSIST_ROOT: &[u8; 34] = b"NETWORK_ROOT_PERSISTANCE_STRUCTURE";

#[derive(Serialize, Deserialize)]
struct PersistentRoot {
	network: String,
	name: String,
}

impl PersistentRoot {
	fn create() -> PersistentRoot {
		// Placeholder
		// TODO: look for settings
		PersistentRoot {
			network: "example.com".to_string(),
			name: "starter".to_string()
		}
	}
	fn find_or_create() -> PersistentRoot {
		println!("PersistentRoot find_or_create");
		let obj_tree = DB.open_tree(BOOT_TREE)
			.expect("Failure opening the object tree");
		match obj_tree.get(NET_PERSIST_ROOT) {
			Ok(op) => match op {
				Some(v) => {
					println!("Deserialize existing");
					ron::de::from_bytes(&v[..]).expect("PersistantRoot corruption")
				},
				None => {
					let out = PersistentRoot::create();
					let mut owriter: Vec<u8> = Vec::new();
					ron::ser::to_writer(
						&mut owriter,
						&out
					).expect("Could not serialize PersistentRoot");
					println!("create new");
					obj_tree.insert(
						NET_PERSIST_ROOT,
						owriter
					).expect("Could not init PersistentRoot");
					obj_tree.flush().expect("could not flush");
					out
				}
			},
			Err(_) => panic!("Sled error")
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
struct RootFragment {
	level: u8,
	#[serde(with = "serde_bytes")]
	slug: Vec<u8>,
}

#[typetag::serde]
impl Fragment for RootFragment {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn get_slug(&self) -> Vec<u8> {
		self.slug.clone()
	}

	fn get_handler(&self) -> Vec<u8> {
		String::from("Root").into_bytes()
	}

	fn get_format(&self) -> Vec<u8> {
		Vec::new()
	}

	fn get_version(&self) -> u64 {
		0
	}

	fn inner_borrow(&self) -> &dyn Fragment {
		self
	}

	fn inner_clone(&self) -> Arc<dyn Fragment+Send+Sync> {
		Arc::new(self.clone())
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Root {
	level: u8,
	network: Vec<u8>,
	name: Vec<u8>,
	//conn_cache: HashMap<[u8; 128], Handler>
}

impl Root {
	pub fn find_or_create<'a>() -> Root {
		println!("Root find_or_create");
		let proot = PersistentRoot::find_or_create();
		Root {
			level: 0,
			network: proot.network.into_bytes(),
			name: proot.name.into_bytes(),
		}
	}
}

#[typetag::serde]
impl Node for Root {
	fn get_nodes(self, frag:&dyn Fragment) -> Result<Vec<Box<dyn Node+Send+Sync>>, Box<dyn Error>> {
		assert!(self.level < 2);
		if self.level == 0 {
			if frag.get_slug() == self.network {
				let mut out = self.clone();
				out.level = 1;
				Ok(vec![Box::new(out)])
			} else {
				Err(
					Box::new(
						OperationError::NotImplemented(
							"Query forwarding not yet implemented"
						)
					)
				)
			}
		} else {
			if frag.get_slug() == self.name {
				let mut out = self.clone();
				out.level = 1;
				match NativeNodeV1::get_root(frag) {
					Ok(r) => Ok(vec![r]),
					Err(_) => {
						NativeNodeV1::create_root(frag)?;
						Ok(vec![NativeNodeV1::get_root(frag)?])
					}
				}
			} else {
				Err(
					Box::new(
						OperationError::NotImplemented(
							"Query forwarding not yet implemented"
						)
					)
				)
			}
		}
	}

	fn list_nodes(self) -> Vec<Box<dyn Fragment>> {
		todo!()
	}

	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement read")))
	}

	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement write")))
	}

	fn create_node(self, _:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement create_node")))
	}

	fn move_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement move_node")))
	}

	fn link_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement link_node")))
	}

	fn unlink_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		Err(Box::new(OperationError::NotImplemented("Network does not implement unlink_node")))
	}
}