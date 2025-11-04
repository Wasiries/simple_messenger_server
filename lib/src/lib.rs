// use std::net::{TcpListener, TcpStream};
// use std::thread;
// use std::io::{Read, Write, Error};

// pub fn enter() -> String {
//     let mut value = String::new();
//     std::io::stdin().read_line(&mut value).unwrap();
//     let value: String = value.trim().parse().unwrap();
//     return value;
// }

// fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
//     println!("Incoming connection from: {}", stream.peer_addr()?);
//     let mut buf = [0; 1024];
//     loop {
//         let bytes_read = stream.read(&mut buf)?;
//         if bytes_read == 0 { 
//             return Ok(());
//         }
//         if &buf[..bytes_read] == "$exit$".as_bytes() {
//             return Ok(());
//         }
//         let s = String::from_utf8_lossy(&buf[..bytes_read]);
//         println!("{}", s);
//     }
// }

// pub fn initialize(ip_: String) -> Result<TcpListener, Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind(&ip_[..])?;
//     return Ok(listener);
// }

// pub fn begining() {
    
// }