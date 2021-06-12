use {
	flume::{unbounded, Receiver, Sender},
	crate::{message::Message, network::Root}
};

pub fn broker_loop(root: Root, sender: Sender<Message>, receiver: Receiver<Message>) {
	loop {
		receiver.recv().unwrap();
	}
}