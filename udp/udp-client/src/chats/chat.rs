use crate::messages::message::Message;
use std::net::SocketAddrV4;

pub struct Chat {
    pub people: Vec<SocketAddrV4>,
    pub messages: Vec<Message>,
}

impl Chat {
    pub fn new(msg: Message, ppl: Vec<SocketAddrV4>) -> Self {
        Chat {
            people: ppl,
            messages: vec![msg],
        }
    }
}
