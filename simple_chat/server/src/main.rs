use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<(SocketAddr, String)>();

    println!("Server started on {}", LOCAL);
    println!("Waiting for clients...");

    loop {
        if let Ok((socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();

            clients.push(socket.try_clone().expect("Failed to clone client socket"));

            thread::spawn(move || read_from_socket(socket, addr, tx));
        }

        if let Ok(msg) = rx.try_recv() {
            clients = broadcast_and_drop_disconnected(clients, msg);
        }

        sleep();
    }
}

fn read_from_socket(mut socket: TcpStream, addr: SocketAddr, tx: Sender<(SocketAddr, String)>) {
    loop {
        let mut buff = vec![0; MSG_SIZE];

        match socket.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid message");

                println!("{}: {}", addr, msg);
                tx.send((addr, msg)).expect("Failed to send message");
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("{}: Client disconnected", addr);
                break;
            }
        }

        sleep();
    }
}

fn broadcast_and_drop_disconnected(clients: Vec<TcpStream>, msg: (SocketAddr, String)) -> Vec<TcpStream> {
    let (sender_addr, text) = msg;

    let mut buff = text.into_bytes();
    buff.resize(MSG_SIZE, 0);

    clients.into_iter().filter_map(|mut client|  {
        match client.peer_addr() {
            Ok(addr) if addr != sender_addr => {
                client.write_all(&buff).map(|_| client).ok()
            },
            Ok(_) => Some(client),
            Err(_) => None
        }
    }).collect()
}

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}