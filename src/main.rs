mod node;
mod path;
mod nativestore;
mod network;
mod operror;
mod packet;
mod message;
mod broker;
mod errormessage;

#[macro_use]
extern crate lazy_static;

use {
	std::{thread, time},
	flume::{unbounded, Receiver, Sender},
	network::Root,
	message::Message,
	packet::Packet,
	broker::broker_loop
};

const BOOT_TREE: &[u8; 15] = b"SYS_BOOT_CONFIG";

lazy_static! {
	static ref DB: sled::Db = sled::open("ode_store.db")
		.expect("Error opening the ODE store");
}

fn main() {
	env_logger::init();
	let root: Root = Root::find_or_create();
	let (s, r): (Sender<Packet>, Receiver<Packet>) = unbounded();
	log::info!("starting workers");
	for _ in 0..num_cpus::get() {
		let tl_root = root.clone();
		let tl_s = s.clone();
		let tl_r = r.clone();
		thread::spawn(|| {broker_loop(tl_root, tl_s, tl_r)});
	}
}
