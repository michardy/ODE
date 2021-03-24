use crate::{
	node::{Node,Object},
	path::{Fragment,Path},
	operror::OperationError,
	DB,
	BOOT_TREE
};

use std::error::Error;

use {
	sha3::{Digest, Sha3_256},
	serde::{Serialize, Deserialize}
};

const NET_PERSIST_ROOT: &[u8; 34] = b"NETWORK_ROOT_PERSISTANCE_STRUCTURE";

#[derive(Serialize, Deserialize)]
struct PersistantRoot {
	network: String,
	name: String,
}

impl PersistantRoot {
	fn create() -> PersistantRoot {
		// Placeholder
		// TODO: look for settings
		PersistantRoot {
			network: "example.com".to_string(),
			name: "starter".to_string()
		}
	}
	fn find_or_create() -> PersistantRoot {
		let obj_tree = DB.open_tree(BOOT_TREE)
			.expect("Failure opening the object tree");
		match obj_tree.get(NET_PERSIST_ROOT) {
			Ok(op) => match op {
				Some(v) => {
					ron::de::from_bytes(&v[..]).expect("PersistantRoot corruption")
				},
				None => {
					PersistantRoot::create()
				}
			},
			Err(_) => panic!("Sled error")
		}
	}
}


#[derive(Serialize, Deserialize)]
struct Root {
	network: Vec<u8>,
	name: Vec<u8>,
	//conn_cache: HashMap<[u8; 128], Handler>
}

impl Root {
	fn find_or_create<'a>() -> Root {
		let proot = PersistantRoot::find_or_create();
		Root {
			network: proot.network.into_bytes(),
			name: proot.name.into_bytes(),
		}
	}
}

#[typetag::serde]
impl Node for Root {
    fn get_node(self, frag:&dyn Fragment) -> Box<dyn Node> {
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
}