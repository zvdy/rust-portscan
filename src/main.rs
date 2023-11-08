use std::env;
use std::net::{IpAddr, TcpStream};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <ip>", args[0]);
        process::exit(1);
    }

    let ip = &args[1];

    for port in 1..65535 {
        let ip_addr = IpAddr::from(ip.parse::<std::net::Ipv4Addr>().unwrap());
        let stream = TcpStream::connect((ip_addr, port));

        match stream {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => {}
        }
    }
}