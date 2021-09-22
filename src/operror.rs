use std::error::Error;

use thiserror::Error;

use crate::{message::Message, path::Fragment};

#[derive(Error, Debug)]
pub enum OperationError {
	#[error("{0}")]
	InternalStoreError(&'static str),

	#[error("{0}")]
	BadMessage(&'static str),

	#[error("{0}")]
	NotImplemented(&'static str),

	#[error("key was not found")]
	KeyNotFound,

	#[error("Escaped from {0}")]
	ErrorEscape(&'static str)
}