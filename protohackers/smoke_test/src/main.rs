use std::io::{Read, Write};
use std::net::{Shutdown::Both, TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

const IP_ADDRESS: &str = "0.0.0.0:7777";
const MAX_CONNECTIONS: usize = 5;

fn main() {
    let server = TcpListener::bind(IP_ADDRESS).expect("Failed to bind to IP address");

    let mut connections: Vec<JoinHandle<_>> = Vec::with_capacity(MAX_CONNECTIONS);

    println!("Listening on {}", IP_ADDRESS);

    loop {
        if let Ok((stream, address)) = server.accept() {
            connections.retain(|handle| !handle.is_finished());

            if connections.len() + 1 > MAX_CONNECTIONS {
                println!("Maximum connections reached");
                stream.shutdown(Both).unwrap_or(());
                continue;
            }

            println!("Accepted connection from {}", address);

            let handle = thread::spawn(move || handle_echo(stream));
            connections.push(handle);
        }
    }
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
