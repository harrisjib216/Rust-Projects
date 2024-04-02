use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX_PORT: u16 = 65535;
struct Arguments {
    thread_count: u16,
    ip_addr: IpAddr,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            // not enough params
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            // too many params
            return Err("Too many arguments supplied");
        } else if args[1].contains("-h") {
            // show help message
            return Err("Help");
        } else if args[1].contains("-j") {
            // parse threads and ip address
            let thread_count: u16 = args[2].parse().unwrap();

            let ip_addr: &str = args[3].trim();
            if let Ok(ip_addr) = IpAddr::from_str(ip_addr) {
                return Ok(Arguments {
                    thread_count,
                    ip_addr,
                });
            }

            return Err("Invalid thread count or ip address");
        } else {
            // handle default case
            let ip_addr = args[1].trim();

            if let Ok(ip_addr) = IpAddr::from_str(ip_addr) {
                return Ok(Arguments {
                    thread_count: 4,
                    ip_addr,
                });
            }

            return Err("Help");
        }
    }
}

fn show_help() {
    println!("Usage: -h or -help to show help, -j [positive integer] for thread count");
}

fn scan_port(tx: Sender<u16>, start_port: u16, addr: IpAddr, thread_count: u16) {
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

        if (MAX_PORT - port) <= thread_count {
            break;
        }
        port += thread_count;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_name = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("Help") {
            show_help();
            process::exit(0);
        } else {
            eprintln!(
                "{} had trouble parsing your aguments. Generated this error below\n{}",
                program_name, err
            );
            process::exit(1);
        }
    });

    let ip_addr = arguments.ip_addr;
    let thread_count = arguments.thread_count;

    let (tx, rx) = channel();

    for index in 0..thread_count {
        let tx = tx.clone();

        thread::spawn(move || {
            scan_port(tx, index, ip_addr, thread_count);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    for v in out {
        println!("{} is open", v);
    }
}

/*
arguments
exe -h
exe -j thread_count ip_addr
exe ip_addr
*/
