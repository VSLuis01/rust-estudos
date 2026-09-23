use std::io::{self, ErrorKind, Read, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let client = TcpStream::connect(LOCAL).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || send_message(tx));

    handle_connection(client, rx);

    println!("Bye bye!");
}

fn handle_connection(mut client: TcpStream, rx: Receiver<String>) {
    loop {

        match try_read_from_server(&mut client) {
            Ok(Some(msg)) => print_received_message(&msg),
            Ok(None) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        match try_write_to_server(&mut client, &rx) {
            Ok(Some(_msg)) => (),
            Ok(None) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }


        thread::sleep(Duration::from_millis(100));
    }
}

fn print_received_message(msg: &str) {
    print!("\r\x1b[2K");
    println!("Message received: {}", msg);
    print!("You: ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn try_read_from_server(client: &mut TcpStream) -> Result<Option<String>, &'static str> {
    let mut buff = vec![0; MSG_SIZE];

    match client.read_exact(&mut buff) {
        Ok(_) => {
            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

            Ok(String::from_utf8(msg).ok())
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            Ok(None)
        },
        Err(_) => {
            Err("An error occurred, disconnecting from server")
        }
    }
}

fn try_write_to_server(client: &mut TcpStream, rx: &Receiver<String>) -> Result<Option<String>, &'static str> {
    match rx.try_recv() {
        Ok(msg) => {
            let mut buff = msg.clone().into_bytes();
            buff.resize(MSG_SIZE, 0);
            client.write_all(&buff).expect("Failed to write to server");

            Ok(Some(msg))
        }
        Err(TryRecvError::Empty) => Ok(None),
        Err(TryRecvError::Disconnected) => Err("Disconnected"),
    }
}

fn send_message(tx: Sender<String>) {
    loop {
        print!("You: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Failed to read line");
        let msg = buff.trim().to_string();

        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
}
