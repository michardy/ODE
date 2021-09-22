//! Defining interface for graph nodes

use {
	crate::path::Fragment,
	std::error::Error
};

/// Trait describing all nodes in the ODE tree/graph
/// Note: all implementors should also implement Serialize and
/// Deserialize because they might be put into the nativestore
/// or sent over the network
/// If you intend on implementing a graph store keep in mind
/// that fragments don't have parent state by default
#[typetag::serde(tag = "type")]
pub trait Node {
	fn get_nodes(self, _:&dyn Fragment) -> Result<Vec<Box<dyn Node+Send+Sync>>, Box<dyn Error>>;
	fn list_nodes(self) -> Vec<Box<dyn Fragment>>;
	fn create_node(self, _:&dyn Fragment) -> Result<Box<dyn Node>, Box<dyn Error>>;
	fn move_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>>;
	fn link_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>>;
	fn unlink_node(self, _:&dyn Fragment) -> Result<(), Box<dyn Error>>;
	fn read(self, start: usize, len:usize) -> Result<Vec<u8>, Box<dyn Error>>;
	fn write(self, start: usize, data: Vec<u8>) -> Result<usize, Box<dyn Error>>;
}

pub trait Object{
}