use clap::Parser;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::thread;

/// A Simple port scanner
#[derive(Parser)]
struct Arguments {
    /// IP address to scan
    #[arg(short = 'a', long = "addr")]
    ip_addr: IpAddr,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(u16).range(1..))]
    threads: u16,
}

fn main() {
    let args = Arguments::parse();

    let num_threads = args.threads;
    let ip_addr = args.ip_addr;

    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, ip_addr, num_threads)
        });
    }

    let mut open_ports = vec![];
    drop(tx);

    for port in rx {
        open_ports.push(port);
    }

    println!();

    open_ports.sort();
    for port in open_ports {
        println!("Port {} is open", port);
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (u16::MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}
