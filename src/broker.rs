use {
	flume::{unbounded, Receiver, Sender},
	crate::{
		packet::Packet,
		message::Message,
		network::Root,
		errormessage::ErrorMessage
	},
	std::error::Error
};

pub fn broker_loop(root: Root, sender: Sender<Packet>, receiver: Receiver<Packet>) {
	loop {
		let packet = receiver.recv().unwrap();
		let route_status = &packet.route();
		match route_status {
			Ok(pv) => {
				for p in pv {
					sender.send(*p);
				}
			},
			Err(e) => {
				let err = Box::new(e.to_string() as ErrorMessage);
				match packet.new_with_source(err, root) {
					Ok(p) => sender.send(p).unwrap(),
					Err(e) => log::error!(
						"Failure routing error: {}", e
					)
				}
			}
		}
	}
}