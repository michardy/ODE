mod node;
mod path;
mod nativestore;
mod network;
mod operror;

#[macro_use]
extern crate lazy_static;

const BOOT_TREE: &[u8; 15] = b"SYS_BOOT_CONFIG";

lazy_static! {
	static ref DB: sled::Db = sled::open("ode_store.db")
		.expect("Error opening the ODE store");
}

fn main() {
    println!("Hello, world!");
}
