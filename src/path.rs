use {
	serde::{Serialize, Deserialize},
	serde_big_array::BigArray
};

#[derive(Serialize, Deserialize)]
pub struct Fragment {
	/// ID of IO handler
	/// Adding a 1 to the highest bit causes retention
	/// to backing store
	handler: u64,
	/// Path ID fragment
	#[serde(with = "BigArray")]
	slug: [u8;128],
	/// Optional interpreter
	/// Zero for any
	interp: u64
}

pub type Path = Vec<Fragment>;