//! The network store is a tree based store
//! It is the root of the ODE tree.
//! All instance Nativestores are subtrees of it

use {
	crate::{
		node::{Node,Object},
		path::{Fragment,Path},
		operror::OperationError,
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
pub struct Root {
	network: Vec<u8>,
	name: Vec<u8>,
	//conn_cache: HashMap<[u8; 128], Handler>
}

impl Root {
	pub fn find_or_create<'a>() -> Root {
		println!("Root find_or_create");
		let proot = PersistentRoot::find_or_create();
		Root {
			network: proot.network.into_bytes(),
			name: proot.name.into_bytes(),
		}
	}
}

#[typetag::serde]
impl Node for Root {
	fn get_node(self, frag:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		todo!()
	}

	fn get_nodes(self) -> Vec<Box<dyn Fragment>> {
		todo!()
	}

	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>> {
		todo!()
	}

	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>> {
		todo!()
	}

	fn create_node(self, _:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>> {
		todo!()
	}

	fn move_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn link_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn unlink_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>> {
		todo!()
	}
}