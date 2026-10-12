use std::io::{Read, Write};
use std::net::{Shutdown::Both, TcpStream};
use tcp_server::serve;

const IP_ADDRESS: &str = "0.0.0.0:7777";
const MAX_CONNECTIONS: usize = 5;

fn main() {
    serve(IP_ADDRESS, MAX_CONNECTIONS, handle_echo)
}

fn handle_echo(mut stream: TcpStream) {
    let address = stream.peer_addr().unwrap();
    let mut buffer = Vec::new();

    if let Err(_) = stream.read_to_end(&mut buffer) {
        stream.shutdown(Both).unwrap();
        println!("Connection closed: {}", address);
        return;
    }

    stream.write_all(&buffer).unwrap();
    stream.shutdown(Both).unwrap();
    println!("Connection closed: {}", address);
}
