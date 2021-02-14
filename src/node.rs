use crate::path;

pub trait Node{
	fn get_node(self, path: path::Path) -> Box<dyn Node>;
}