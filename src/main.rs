mod node;
mod path;
mod nativestore;
mod network;
mod operror;
mod packet;
mod message;

#[macro_use]
extern crate lazy_static;

use network::Root;

use std::{thread, time};

const BOOT_TREE: &[u8; 15] = b"SYS_BOOT_CONFIG";

lazy_static! {
	static ref DB: sled::Db = sled::open("ode_store.db")
		.expect("Error opening the ODE store");
	//static ref ROOT: Root = Root::find_or_create();
}

fn main() {
	println!("Hello, world!");
	let root: Root = Root::find_or_create();
}
