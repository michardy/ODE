pub trait Message {
	fn get_type(&self) -> &'static str;
}