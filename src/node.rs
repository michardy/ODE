use crate::path::Fragment;

use std::error::Error;

/// Trait describing all nodes in the ODE tree
/// Note: all implementors should also implement Serialize and
/// Deserialize because they might be put into the nativestore
/// or sent over the network
#[typetag::serde(tag = "type")]
pub trait Node {
	fn get_node(self, _:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>>;
	fn get_nodes(self) -> Vec<Box<dyn Fragment>>;
	//fn get_object(self) -> Option<Box<dyn Object>>;
	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>>;
	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>>;
}

pub trait Object{
}