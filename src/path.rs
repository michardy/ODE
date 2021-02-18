use {
	byteorder::{BigEndian, LittleEndian},
	zerocopy::{
		byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned, U16, U32,
	}
};

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct Fragment {
	/// ID of IO handler
	/// Adding a 1 to the highest bit causes retention
	/// to backing store
	handler: U64<BigEndian>,
	/// Path ID fragment
	slug: [u8;128],
	/// Optional interpreter
	/// Zero for any
	interp: U64<BigEndian>
}

pub type Path = Vec<Fragment>;