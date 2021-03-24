use crate::path::Fragment;

use std::error::Error;

#[typetag::serde(tag = "type")]
pub trait Node {
	fn get_node(self, _:&dyn Fragment) -> Box<dyn Node>;
	fn get_nodes(self) -> Vec<Box<dyn Fragment>>;
	//fn get_object(self) -> Option<Box<dyn Object>>;
	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>>;
	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>>;
}

pub trait Object{
}