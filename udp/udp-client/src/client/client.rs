use std::net::{SocketAddrV4, UdpSocket};

use crate::{
    chats::chat::Chat,
    messages::message::Message,
    requests::{
        request::{Request, RequestKind},
        response::{Response, ResponseKind},
    },
};

pub struct Client {
    pub addr: SocketAddrV4,
    pub socket: UdpSocket,
    pub remote_addr: SocketAddrV4,
    pub remote_socket: UdpSocket,
    pub available_people: Vec<SocketAddrV4>,
    pub chats: Vec<Chat>,
}

impl Client {
    fn handle_response(&mut self, resp: Response) {
        match resp.kind {
            ResponseKind::AVAILABILITY_RESPONSE { available_people } => {
                self.available_people = available_people
            }
            ResponseKind::COMING_RESPONSE { welcome } => {
                println!("Greetings from server: {}", welcome)
            }
            ResponseKind::MESSAGE_RESPONSE { message } => self.handle_message(message, resp.dst),
        }
    }

    fn handle_message(&mut self, msg: Message, mut dst: Vec<SocketAddrV4>) {
        // verifier si la liste de chats est vide : si oui, alors il faut en faire un new
        // il faudra penser a se retirer de la liste dst a un moment
        dst.retain(|addr| *addr != self.addr);
        dst.sort();
        // on fait un match entre dst et chat.people
        for chat in &mut self.chats {
            // faut faire une logique de set, pas de liste la
            chat.people.sort();
            let new_message = msg.clone();
            if chat.people == dst {
                chat.messages.push(new_message);
                return;
            } else {
                continue;
            }
        }
        self.chats = vec![Chat::new(msg, dst)];
    }

    fn send_request(&self, msg: String, ppl: Vec<SocketAddrV4>, kind: RequestKind) {
        let request = Request {
            src: self.addr,
            dst: ppl,
            kind: kind,
            message: msg,
        };
        let bytes = match request.serialize() {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error !");
                return;
            }
        };
        self.remote_socket
            .send_to(&bytes, self.remote_addr)
            .expect("Couldn't send request");
    }

    // parcours les chats pour savoir si un chat contient les memes dst :
    // si non : chat new
    // si oui : append du message
}
