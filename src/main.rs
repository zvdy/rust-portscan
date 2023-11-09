use std::env;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let ip = match args.get(1) {
        Some(ip) => match ip.parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Invalid IP address");
                process::exit(1);
            }
        },
        None => {
            eprintln!("Usage: portscanner <ip> [options]");
            process::exit(1);
        }
    };
    let mut start_port = 1;
    let mut end_port = 65535;
    let mut scan_type = ScanType::Default;
    let mut verbose = false;
    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "-r" => {
                scan_type = ScanType::Range;
            }
            "-s" => {
                scan_type = ScanType::Stealth;
            }
            "-v" => {
                verbose = true;
            }
            _ => {
                let ports: Vec<&str> = arg.split('-').collect();
                if ports.len() == 2 {
                    start_port = match ports[0].parse::<u16>() {
                        Ok(port) => port,
                        Err(_) => {
                            eprintln!("Invalid start port");
                            process::exit(1);
                        }
                    };
                    end_port = match ports[1].parse::<u16>() {
                        Ok(port) => port,
                        Err(_) => {
                            eprintln!("Invalid end port");
                            process::exit(1);
                        }
                    };
                } else {
                    eprintln!("Invalid option: {}", arg);
                    process::exit(1);
                }
            }
        }
    }
    // Perform port scan based on scan type
    match scan_type {
        ScanType::Default => {
            // Default scan: scan all ports in range
            for port in start_port..=end_port {
                let addr = SocketAddr::new(ip, port);
                match TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(500)) {
                    Ok(_) => println!("Port {} is open", port),
                    Err(_) => {
                        if verbose {
                            println!("Port {} is closed", port);
                        }
                    }
                }
            }
        }
        ScanType::Range => {
            // Range scan: scan ports in specified range
            for port in start_port..=end_port {
                let addr = SocketAddr::new(ip, port);
                let result = TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(500));
                match result {
                    Ok(_) => println!("Port {} is open", port),
                    Err(_) => {
                        if verbose {
                            println!("Port {} is closed", port);
                        }
                    }
                }
            }
        }
        ScanType::Stealth => {
            // Stealth scan: scan ports in specified range, but do not print closed ports
            for port in start_port..=end_port {
                let addr = SocketAddr::new(ip, port);
                let result = TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(500));
                match result {
                    Ok(_) => {},
                    Err(_) => {
                        if verbose {
                            println!("Port {} is closed or filtered: {}", port, get_service_name(port));
                        }
                    }
                }
            }
        }
    }
}

enum ScanType {
    Default,
    Range,
    Stealth,
}

// Get service name for well-known ports
fn get_service_name(port: u16) -> String {
    match port {
        21 => "FTP".to_string(),
        22 => "SSH".to_string(),
        23 => "Telnet".to_string(),
        25 => "SMTP".to_string(),
        53 => "DNS".to_string(),
        80 => "HTTP".to_string(),
        110 => "POP3".to_string(),
        143 => "IMAP".to_string(),
        443 => "HTTPS".to_string(),
        587 => "SMTPS".to_string(),
        993 => "IMAPS".to_string(),
        995 => "POP3S".to_string(),
        _ => "Unknown".to_string(),
    }
}