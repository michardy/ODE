use std::borrow::Borrow;

use crate::{operror::OperationError, path::Fragment};

use {
	crate::{
		message::Message,
		path::Path,
		node::Node,
		network::Root
	},
	std::error::Error
};

pub struct Packet<'a> {
	parent: Box<dyn Node+Send+Sync>,
	target: &'a Path,
	source: Option<Path>,
	data: Box<dyn Message+Send+Sync>
}

fn clone_path<'a>(path: &Path) -> &'a Path {
	let out: Path = Vec::with_capacity(path.len());
	for f in path {
		out.push(f.inner_clone());
	}
	&out
}

impl<'a> Packet<'a> {
	pub fn route<'b>(self) -> Result<Vec<Packet<'b>>, Box<dyn Error>> {
		let mut path = *self.target;
		let frag = &path.remove(0);
		let out_nodes: Vec<Box<dyn Node+Send+Sync>> = self.parent.get_nodes(frag.inner_borrow())?;
		let mut out: Vec<Packet> = Vec::with_capacity(out_nodes.len());
		for node in out_nodes {
			out.push(Packet{
				parent: node,
				target: &path,
				source: self.source,
				data: self.data,
			})
		}
		Ok(out)
	}
	pub fn new_with_source<'b>(self, message: Box<dyn Message+Send+Sync>, root: Root) -> Result<Packet<'b>, Box<dyn Error>> {
		match self.source {
			Some(source) => {
				Ok(
					Packet {
						parent: Box::new(root),
						target: clone_path(&source),
						source: None,
						data: message
					}
				)
			},
			None => {
				Err(
					Box::new(
						OperationError::ErrorEscape("")
					)
				)
			}
		}
		
	}
}