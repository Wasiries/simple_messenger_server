use std::net::{*};
use std::thread::Thread;

pub struct User {
    name: String,
    ip: String,
    listener: TcpListener,
}

impl User {
    fn enter_name() -> String {
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        let value: String = value.trim().parse().unwrap();
        return value;
    }
    fn enter_ip() -> String {
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        let value: String = value.trim().parse().unwrap();
        return value;
    }
    pub fn new() -> Self {
        let name = User::enter_name();
        let ip = User::enter_ip();
        let listener = TcpListener::bind(&ip[..]).unwrap();
        return User {
            name,
            ip,
            listener,
        };
    }
    pub fn handle_server(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

        return Ok(());
    }
    pub fn start_seerver(&self) -> Result<(), Box<dyn std::error::Error>> {
        let closure = || {
            for stream in self.listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let _ = self.handle_server(stream);
                    },
                    Err(_) => {
                        println!("Connection error");
                    }
                }
            }
        };
        // let proc = std::thread::spawn(closure);
        return Ok(());
    }
}