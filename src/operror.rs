use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
	#[error("{0}")]
	InternalStoreError(&'static str),

	#[error("key was not found")]
	KeyNotFound
}