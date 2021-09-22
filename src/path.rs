use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait Fragment {
	fn as_any(&self) -> &dyn Any;
	fn get_slug(&self) -> Vec<u8>;
	fn get_handler(&self) -> Vec<u8>;
	fn get_format(&self) -> Vec<u8>;
	fn get_version(&self) -> u64;
	fn equal(&self, other: &dyn Fragment) -> bool {
		return
			// TODO: Check that this compares values not references
			(self.get_slug() == other.get_slug()) &&
			(self.get_handler() == other.get_handler()) &&
			(self.get_format() == other.get_format()) &&
			(self.get_version() == other.get_version());
	}
	fn inner_borrow(&self) -> &dyn Fragment;
	fn inner_clone(&self) -> Box<dyn Fragment+Send+Sync>;
}

pub type Path = Vec<Box<dyn Fragment+Send+Sync>>;