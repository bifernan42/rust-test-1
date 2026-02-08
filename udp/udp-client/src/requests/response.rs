use crate::messages::message::Message;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;

// faudra enlever les uppercase
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseKind {
    AVAILABILITY_RESPONSE { available_people: Vec<SocketAddrV4> },
    MESSAGE_RESPONSE { message: Message },
    COMING_RESPONSE { welcome: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub dst: Vec<SocketAddrV4>, // permet de retrouver le Chat, mais c'est pas terrible
    pub src: SocketAddrV4,
    pub kind: ResponseKind,
}

impl Response {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, bincode::Error> {
        // comment securiser la deserialization ?
        bincode::deserialize(buf)
    }
}
