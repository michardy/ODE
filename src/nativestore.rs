use crate::{
	node::{Node,Object},
	path::{Fragment,Path},
	operror::OperationError,
	DB
};

use std::error::Error;

use {
	byteorder::{BigEndian, LittleEndian},
	zerocopy::{
		byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned, U16, U32, U128
	}
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

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
/// Fragment for nativestore
/// Since we know the storage we don't track it
struct NativeFragment{
	parent: U128<LittleEndian>,
	slug: [u8; 128],
	format: U64<BigEndian>
}

pub struct NativeNode{
	/// Key of parent
	parent: Option<NativeFragment>,
	/// Internal ID
	/// First part of child keys
	id: U128<LittleEndian>,
	/// Second part of child keys
	children: Vec<Fragment>,
	/// Hashes of data blocks or Objects
	data: Vec<[u8; 256]>
}

impl Node for NativeNode {
	fn get_node(self, fragment: Fragment) -> Box<dyn Node> {
		Box::new(self)
	}
	fn get_nodes(self) -> Vec<Fragment> {
		self.children
	}
	fn read(self, start: usize, len: usize) -> Result<Vec<u8>, Box<dyn Error>> {
		if (self.data.len() > 0) {
			let mut index = start/BLOCK_SIZE;
			let mut off = start%BLOCK_SIZE;
			let obj_tree = DB.open_tree(OBJT_TREE)
				.expect("Failure opening the object tree");
			let mut out: Vec<u8> = Vec::new();
			while (index >= self.data.len() && out.len() < len) {
				match obj_tree.get(self.data[index]) {
					Ok(op) => match op {
						Some(v) => out.append(&mut(v.subslice(
								off,
								if len-off > 1024 {len-off} else {1024}
							)[..]).into()),
						None => return Err(
							Box::new(OperationError::InternalStoreError(
								"Nativestore inconsistancy (rebuild indexes)"
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
}


impl Object for Vec<Vec<u8>> {
	
}