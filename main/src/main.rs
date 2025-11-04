
use std::{
    net::{TcpListener, TcpStream},
    thread,
    io::{self, Read, Write},
    sync::mpsc,
};

#[allow(dead_code)]
fn encryption(value: &[u8]) -> Vec<u8> {
    let ans = Vec::from(value);
    return ans;
}
#[allow(dead_code)]
fn decryption(value: &[u8]) -> Vec<u8> {

    let ans = Vec::from(value);
    return ans;
}


fn main() {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    ip = ip.trim().parse().unwrap();

    let listener = TcpListener::bind(&ip[..]).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let (sender, receiver) = mpsc::channel();
        let (interruption_s, interruption_r) = mpsc::channel();
        let closure = move || {
            chating(sender, interruption_r);
        };
        let handle = thread::spawn(closure);
        handle_client(stream, receiver);
        let _ = interruption_s.send(true);
        let _ = handle.join();
    }
}

fn chating(sender: mpsc::Sender<String>, interruption: mpsc::Receiver<bool>) {
    let mut value = String::new();
    loop {
        if let Ok(_) = interruption.try_recv() {
            break;
        }
        io::stdin().read_line(&mut value).unwrap();
        value = value.trim().parse().unwrap();
        let _ = sender.send(value.clone());
        value = String::new();
    }
}

fn handle_client(mut stream: TcpStream, receiver: mpsc::Receiver<String>) {
    let mut buffer: [u8; 2048] = [0; 2048];
    loop {
        if let Ok(value) = receiver.try_recv() {
            let _ = stream.write(value.as_bytes());
        }
        match stream.read(&mut buffer) {
            Ok(buffer_size) => {
                if buffer_size == 0 {
                    println!("Client has disconnected");
                    break;
                } else {
                    let value = &buffer[..buffer_size];
                    let message = String::from_utf8(decryption(value)).unwrap();
                    println!("{}", message);
                }
            },
            Err(e) => {
                println!("Client processing error: {}", e.kind());
                break;
            }
        }
    }
}