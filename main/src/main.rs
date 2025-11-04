
use std::{
    net::{TcpListener, TcpStream},
    thread,
    io::{self, Read, Write},
    sync::mpsc,
};

fn downsize(vector: Vec<u8>) -> Vec<u64> {
    let mut temp = vec![0u64; vector.len() / 8];
    for i in 0..(vector.len() / 8) {
        for j in (0..8).rev() {
            temp[i] *= 256;
            temp[i] += vector[i * 8 + j] as u64;
        }
    }
    return temp;
}

fn extend(mut vector: Vec<u64>) -> Vec<u8> {
    let mut temp = vec![0u8; vector.len() * 8];
    for i in 0..vector.len() {
        for j in 0..8 {
            temp[i * 8 + j] = (vector[i] % 256) as u8;
            vector[i] /= 256;
        }
    }

    return temp;
}

fn powmod(n: u64, k: u64, m: u64) -> u64 {
    let mut ans = 1u64;
    for _ in 0..k {
        ans *= n;
        ans = ans % m;
    }
    return ans;
}

#[allow(dead_code)]
fn encryption(value: &[u8], b: u64, rb: u64) -> Vec<u8> {
    let mut temp = vec![0u64; value.len()];
    for i in 0..value.len() {
        temp[i] = value[i] as u64;
        temp[i] = powmod(temp[i], b, rb);
    }
    let ans = extend(temp);
    return ans;
}
#[allow(dead_code)]
fn decryption(value: &[u8], alpha: u64, ra: u64) -> Vec<u8> {
    let temp = downsize(Vec::from(value));
    let mut ans = vec![0u8; temp.len()];
    for i in 0..temp.len() {
        ans[i] = powmod(temp[i], alpha, ra) as u8;
    }
    return ans;
}

fn without_decryption(value: &[u8]) -> Vec<u8> {
    let temp = downsize(Vec::from(value));
    let mut ans = vec![0u8; temp.len()];
    for i in 0..temp.len() {
        ans[i] = temp[i] as u8;
    }
    return ans;
}

fn alpha_finder(a: u64, fi: u64) -> u64 {
    for i in 1..fi {
        if (a * i) % fi == 1 {
            return i;
        }
    }
    return 0;
}

#[allow(dead_code)]
fn gcd(first: u64, second: u64) -> u64 {
    let mut f = first;
    let mut s = second;
    while s != 0 {
        let t = f % s;
        f = s;
        s = t;
    }
    return f;
}

#[allow(dead_code)]
fn euler(first: u64, second: u64) -> u64 {
    return (first - 1) * (second - 1);
}

#[allow(dead_code)]
fn main() {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    ip = ip.trim().parse().unwrap();
    let mut p1 = String::new();
    io::stdin().read_line(&mut p1).unwrap();
    let p1: u64 = p1.trim().parse().unwrap();
    let mut p2 = String::new();
    io::stdin().read_line(&mut p2).unwrap();
    let p2: u64 = p2.trim().parse().unwrap();
    println!("ra = {}", p1 * p2);
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: u64 = a.trim().parse().unwrap();
    println!("a = {}", a);
    let alpha = alpha_finder(a, euler(p1, p2));
    println!("alpha = {}", alpha);
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b: u64 = b.trim().parse().unwrap();
    println!("b = {}", b);
    let mut rb = String::new();
    io::stdin().read_line(&mut rb).unwrap();
    let rb: u64 = rb.trim().parse().unwrap();
    println!("rb = {}", rb);

    let listener = TcpListener::bind(&ip[..]).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                
                let (sender, receiver) = mpsc::channel();
                let (sinterr, rinterr) = mpsc::channel();
                let (sinterw, rinterw) = mpsc::channel();
                let stream_clone = stream.try_clone();
                let read = move || {
                    reading(stream_clone.unwrap(), rinterr, alpha, p1 * p2);
                };
                let write = move || {
                    writing(stream, receiver, rinterw);
                };
                let read_handle = thread::spawn(read);
                let write_handle = thread::spawn(write);
                let mut input = String::new();

                loop {
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(0) => {
                            break;
                        },
                        Ok(_) => {
                            let message = input.trim().to_string();
                            if message.is_empty() {
                                continue;
                            }
                            print!("\n");
                            if sender.send(message + "\n").is_err() {
                                break;
                            }
                        },
                        Err(error) => {
                            println!("Input error: {}", error.kind());
                            break;
                        }
                    }
                }
                sinterr.send(true).unwrap();
                sinterw.send(true).unwrap();
                let _ = read_handle.join();
                let _ = write_handle.join();

                break;
            }, 
            Err(error) => {
                println!("Connection error: {}", error.kind());
            }
        }
        break;
    }
    println!("Server work finished");
}

fn reading(mut stream: TcpStream, interruption: mpsc::Receiver<bool>, alpha: u64, ra: u64) {
    let mut buffer = [0; 2048];
    loop {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection lost");
                break;
            },
            Ok(buffer_size) => {
                let encrypted = String::from_utf8(without_decryption(&buffer[..buffer_size])).unwrap();
                let message = String::from_utf8(decryption(&buffer[..buffer_size], alpha, ra)).unwrap();
                println!("{}\n{}", encrypted, message);
            },
            Err(error) => {
                println!("Connection error: {}", error.kind());
                break;
            }
        }
    }
}

fn writing(mut stream: TcpStream, receiver: mpsc::Receiver<String>, interruption: mpsc::Receiver<bool>) {
    for message in receiver {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        if let Err(error) = stream.write(message.as_bytes()) {
            println!("Failed to send message due to: {}", error.kind());
            break;
        }
    }
}