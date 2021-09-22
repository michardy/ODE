use crate::message::Message;

pub type ErrorMessage = String;

impl Message for ErrorMessage {
	fn get_type(&self) -> &'static str {
		"ERROR"
	}
}