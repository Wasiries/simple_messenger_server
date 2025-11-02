use std::net::{*};
use std::sync::*;

pub struct User {
    name: String,
    ip: String,
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
        return User {
            name,
            ip,
        };
    }
    fn user_enter(&self, sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut value = String::new();
        loop {
            std::io::stdin().read_line(&mut value)?;
            value = value.trim().parse()?;
            if value == String::from("$exit$") {
                break;
            } else {
                sender.send(value.clone())?;
                value = String::new();
            }
        }

        return Ok(());
    }
    fn server_handle(&self, stream: TcpStream, receiver: mpsc::Receiver<String>, sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
        
        return Ok(());
    }
    pub fn start_server(&'static self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.ip.clone()[..])?;
        let (sender, receiver) = mpsc::channel();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server_closure = move || {
                        let _ = self.server_handle(stream, receiver, sender);
                    };
                    let handle = std::thread::spawn(server_closure);
                    let _ = handle.join();
                    break;
                },
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }
        return Ok(());
    }
}