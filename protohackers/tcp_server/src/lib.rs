use std::net::Shutdown::Both;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

pub fn serve(ip_address: &str, max_connections: usize, on_connection: fn(TcpStream)) {
    let server = TcpListener::bind(ip_address).expect("Failed to bind to IP address");
    let mut connections: Vec<JoinHandle<_>> = Vec::with_capacity(max_connections);

    println!("Listening on {}", ip_address);

    loop {
        if let Ok((stream, address)) = server.accept() {
            connections.retain(|handle| !handle.is_finished());

            if connections.len() + 1 > max_connections {
                println!("Maximum connections reached");
                stream.shutdown(Both).unwrap_or(());
                continue;
            }

            println!("Accepted connection from {}", address);

            let handle = thread::spawn(move || on_connection(stream));
            connections.push(handle);
        }
    }
}