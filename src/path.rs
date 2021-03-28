use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait Fragment {
	fn as_any(&self) -> &dyn Any;
	fn get_slug(&self) -> Vec<u8>;
	fn get_handler(&self) -> Vec<u8>;
	fn get_format(&self) -> Vec<u8>;
	fn get_version(&self) -> u64;
}

pub type Path = Vec<Box<dyn Fragment>>;