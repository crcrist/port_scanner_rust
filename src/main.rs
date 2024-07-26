use std::env;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

/*1. Accept user input for IP address in range
    2. Attempt to connect to each port in given range
    3. Report which ports are open
    4. Use multithreading */
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <ip_address> <start_port> <end_port>", args[0]);
        return;
    }


    let ip = IpAddr::from_str(&args[1]).expect("Invalid IP address");
    let start_port: u16 = args[2].parse().expect("Invalid start port");
    let end_port: u16 = args[3].parse().expect("Invalid end port");

    let (tx, rx) = channel();

    for port in start_port..=end_port {
        let tx = tx.clone();
        thread::spawn(move|| {
            scan_port(ip, port, tx);
        });
    }

    drop(tx);

    let mut open_ports = Vec::new();
    for port in rx {
        open_ports.push(port);
    }

    open_ports.sort();
    println!("Open ports:");
    for port in open_ports {
        println!("{}", port);
    }
}


fn scan_port (ip: IpAddr, port: u16, sender: Sender<u16>) {
    let socket = SocketAddr::new(ip, port);
    match TcpStream::connect_timeout(&socket, Duration::from_millis(200)) { 
        Ok(_) => {
            sender.send(port).expect("Could not send port through channel");
        }
        Err(_) => {}
    }
}