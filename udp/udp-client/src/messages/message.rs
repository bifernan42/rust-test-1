use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: SocketAddrV4,
    pub date: DateTime<Utc>,
    pub content: String,
}
