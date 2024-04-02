use std::net::TcpStream;

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
