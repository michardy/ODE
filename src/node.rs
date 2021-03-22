use crate::path::Fragment;

use std::error::Error;

pub trait Node{
	fn get_node(self, _:Fragment) -> Box<dyn Node>;
	fn get_nodes(self) -> Vec<Fragment>;
	//fn get_object(self) -> Option<Box<dyn Object>>;
	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>>;
	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>>;
}

pub trait Object{
}