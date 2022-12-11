use std::net::{IpAddr, TcpListener, TcpStream};

fn main() {
    for port in 0..=65535 {
        port_is_available(port);
    }
}

fn port_is_available(port: u16) {
    match TcpStream::connect(("127.0.0.1", port)) {
        Ok(_) => {
            println!("{} is open/available", port);
        }
        Err(_) => {}
    }
}

/*use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;

const MAX_PORT: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ip_addr = args[1].trim();

    if let Ok(ip_addr) = IpAddr::from_str(ip_addr) {
        for port_index in 0..MAX_PORT {
            match TcpStream::connect((ip_addr, port_index)) {
                Ok(_) => {
                    println!("port {} is open", port_index);
                }
                Err(_) => {
                    println!("could not read port {}", port_index);
                }
            }
        }
    }
}
*/
/*
arguments
exe -h
exe -j thread_count ip_addr
exe ip_addr
*/
