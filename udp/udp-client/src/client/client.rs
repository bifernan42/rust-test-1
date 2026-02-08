use std::net::SocketAddrV4;

use crate::{
    chats::chat::Chat,
    messages::message::Message,
    requests::response::{Response, ResponseKind},
};

pub struct Client {
    pub available_people: Vec<SocketAddrV4>,
    pub chats: Vec<Chat>,
}

impl Client {
    fn do_something() {}

    fn handle_response(&mut self, resp: Response) {
        match resp.kind {
            ResponseKind::AVAILABILITY_RESPONSE { available_people } => {
                self.available_people = available_people
            }
            ResponseKind::COMING_RESPONSE { welcome } => {
                println!("Greetings from server: {}", welcome)
            }
            ResponseKind::MESSAGE_RESPONSE { message } => print!("OK"),
        }
    }

    fn handle_message(&mut self, msg: Message, dst: Vec<SocketAddrV4>) {
        // verifier si la liste de chats est vide : si oui, alors il faut en faire un new
        if self.chats.len() == 0 {
            // il faudra penser a se retirer de la liste dst a un moment
            self.chats = vec![Chat::new(msg, dst)]
        } else {
            // on fait un match entre dst et chat.people
            for chat in self.chats {
                // faut faire une logique de set, pas de liste la
                if chat == dst {}
            }
        }

        // parcours les chats pour savoir si un chat contient les memes dst :
        // si non : chat new
        // si oui : append du message
    }
}
