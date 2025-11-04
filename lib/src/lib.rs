pub static mut IP: String = String::new();

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

pub fn enter() -> String {
    let mut value = String::new();
    std::io::stdin().read_line(&mut value).unwrap();
    let value: String = value.trim().parse().unwrap();
    return value;
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

pub fn initialize(ip_: String) {
    unsafe {
        IP = ip_;
    }
}

pub fn begining() {
    unsafe {
        let listener = TcpListener::bind(&IP[..]).expect("Could not bind");
        for stream in listener.incoming() {
            match stream {
                Err(e) => { eprintln!("failed: {}", e) }
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }
    }
}