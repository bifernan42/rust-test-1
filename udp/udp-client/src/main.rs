use std::io;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
mod chats;
mod client;
mod messages;
mod requests;
use requests::request::{Request, RequestKind};

fn recv_msg(socket: &UdpSocket, buf: &mut Vec<u8>) -> io::Result<(usize, SocketAddr)> {
    socket.recv_from(buf)
}

fn send_msg(socket: &UdpSocket, buf: &mut Vec<u8>, addr: &SocketAddrV4) -> io::Result<usize> {
    socket.send_to(buf, addr)
}

fn main() -> std::io::Result<()> {
    {
        let src: SocketAddrV4 = "127.0.0.1:34255"
            .parse()
            .expect("unable to parse socket adress");

        let dst: SocketAddrV4 = "127.0.0.1:34254"
            .parse()
            .expect("unable to parse socket adress");

        let socket = UdpSocket::bind("127.0.0.1:34255").expect("Could not bind to address");

        let request: Request = Request {
            src: src,
            dst: vec![dst],
            kind: RequestKind::COMING_REQUEST,
            message: "This is a coming request".to_string(),
        };

        let mut bytes = request.serialize().expect("Failed to serialize");

        let nbytes = send_msg(&socket, &mut bytes, &dst).expect("Could not send message");
        println!("{} bytes message sent to {}", nbytes, dst);

        //let (amt, src) = recv_msg(&socket, &mut buf).expect("Did not recieve data");

        //println!(
        //    "Recieved {} bytes response from {}: {:?}",
        //    amt, src, response
        //);
        //buf.reverse();
    }
    Ok(())
}
