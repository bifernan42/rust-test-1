use std::io;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
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
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind to address");

        let mut buf = vec![0u8; 140];

        let (amt, src) = recv_msg(&socket, &mut buf).expect("Did not recieve data");
        let mut request = Request::deserialize(&buf).expect("Could not read message");

        println!(
            "Recieved {} bytes, from {} : {}",
            buf.len(),
            request.src,
            request.message
        );
    }
    Ok(())
}
