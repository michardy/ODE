use crate::{
    message::Message,
    path::Path
};

pub struct Packet {
    target: Path,
    data: Box<Message>
}