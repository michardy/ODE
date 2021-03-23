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
	serde::{Serialize, Deserialize},
	serde_big_array::BigArray
};

const NET_PERSIST_ROOT: &[u8; 34] = b"NETWORK_ROOT_PERSISTANCE_STRUCTURE";

#[derive(Serialize, Deserialize)]
struct PersistantRoot {
	#[serde(with = "BigArray")]
	network: [u8; 128],
	#[serde(with = "BigArray")]
	name: [u8; 128],
}

impl PersistantRoot {
	fn create() -> PersistantRoot {
		// Placeholder
		// TODO: look for settings
		PersistantRoot {
			network: b"example.com",
			name: b"starter"
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

struct Root {
	p_root: PersistantRoot,
	//conn_cache: HashMap<[u8; 128], Handler>
}

impl Root {
	fn find_or_create() -> Root {
		Root {
			p_root: PersistantRoot::find_or_create() 
		}
	}
}

impl Node for Root {
	
}