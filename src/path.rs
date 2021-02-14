use std::num::NonZeroU64;

pub struct Part {
	/// ID of IO handler
	/// Adding a 1 to the highest bit causes retention
	/// to backing store
	handler: u64,
	/// Path ID fragment
	id: [u8;128],
	/// Optional interpreter
	/// Zero for any
	interp: Option<NonZeroU64>
}

pub type Path = Vec<Part>;