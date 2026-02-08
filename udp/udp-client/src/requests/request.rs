use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestKind {
    LEAVING_REQUEST,
    COMING_REQUEST,
    AVAILABILITY_REQUEST,
    MESSAGE_REQUEST,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub src: SocketAddrV4,
    pub dst: Vec<SocketAddrV4>,
    pub kind: RequestKind,
    pub message: String,
}

impl Request {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, bincode::Error> {
        // comment securiser la deserialization ?
        bincode::deserialize(buf)
    }
}
