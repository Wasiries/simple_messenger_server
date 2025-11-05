
use std::{
    net::{TcpListener, TcpStream},
    thread,
    io::{self, Read, Write},
    sync::mpsc,
};

fn downsize(vector: &[u8]) -> Vec<u64> {
    let chunks = vector.chunks(8);
    let mut result = Vec::new();
    
    for chunk in chunks {
        let mut value = 0u64;
        for &byte in chunk {
            value = (value << 8) | byte as u64;
        }
        if chunk.len() < 8 {
            value <<= 8 * (8 - chunk.len());
        }
        result.push(value);
    }
    
    return result;
}

fn extend(vector: &[u64]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &value in vector {
        let mut temp = value;
        let mut bytes = Vec::new();
        
        for _ in 0..8 {
            bytes.push((temp & 0xFF) as u8);
            temp >>= 8;
        }
        result.extend(bytes.into_iter().rev());
    }
    
    result
}

fn powmod(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    return result;
}

#[allow(dead_code)]
fn encryption(value: &[u8], b: u64, rb: u64) -> Vec<u8> {
    let mut temp = vec![0u64; value.len()];
    for i in 0..value.len() {
        temp[i] = value[i] as u64;
        temp[i] = powmod(temp[i], b, rb);
    }
    let ans = extend(&temp[..]);
    return ans;
}
#[allow(dead_code)]
fn decryption(value: &[u8], alpha: u64, ra: u64) -> Vec<u8> {
    let temp = downsize(&Vec::from(value)[..]);
    let mut ans = vec![0u8; temp.len()];
    for i in 0..temp.len() {
        ans[i] = powmod(temp[i], alpha, ra) as u8;
    }
    return ans;
}


fn without_decryption(value: &[u8]) -> Vec<u8> {
    let temp = downsize(&Vec::from(value)[..]);
    let mut ans = vec![0u8; temp.len()];
    for i in 0..temp.len() {
        ans[i] = (temp[i] % 256) as u8;
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
    println!("Enter ip to host server:");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    ip = ip.trim().parse().unwrap();


    let listener = TcpListener::bind(&ip[..]).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connection was set");
                println!("Enter p1:");
                let mut p1 = String::new();
                io::stdin().read_line(&mut p1).unwrap();
                let p1: u64 = p1.trim().parse().unwrap();
                println!("Enter p2:");
                let mut p2 = String::new();
                io::stdin().read_line(&mut p2).unwrap();
                let p2: u64 = p2.trim().parse().unwrap();
                println!("ra = {}", p1 * p2);
                println!("Enter a:");
                let mut a = String::new();
                io::stdin().read_line(&mut a).unwrap();
                let a: u64 = a.trim().parse().unwrap();
                println!("a = {}", a);
                let alpha = alpha_finder(a, euler(p1, p2));
                println!("alpha = {}", alpha);
                println!("Enter b:");
                let mut b = String::new();
                io::stdin().read_line(&mut b).unwrap();
                let b: u64 = b.trim().parse().unwrap();
                println!("b = {}", b);
                println!("Enter rb:");
                let mut rb = String::new();
                io::stdin().read_line(&mut rb).unwrap();
                let rb: u64 = rb.trim().parse().unwrap();
                println!("rb = {}", rb);
                let mut buf = [0u8; 4096];
                let z = [0u8; 16];
                stream.write(&z).unwrap();
                let v = stream.read(&mut buf).unwrap();
                if v == 16 {
                    stream.write(&buf[..v]).unwrap();
                    println!("You can start chating");
                } else {
                    stream.write(&buf[..v/2]).unwrap();
                    println!("Internal Error");
                    return;
                }
                let (sender, receiver) = mpsc::channel();
                let (sinterr, rinterr) = mpsc::channel();
                let (sinterw, rinterw) = mpsc::channel();
                let stream_clone = stream.try_clone();
                let read = move || {
                    reading(stream_clone.unwrap(), rinterr, alpha, p1 * p2);
                };
                let write = move || {
                    writing(stream, receiver, rinterw, b, rb);
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
    let mut buffer = [0; 4096];
    loop {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection lost");
                return;
            },
            Ok(buffer_size) => {
                let t1 = &without_decryption(&buffer[..buffer_size])[..];
                let encrypted = String::from_utf8_lossy(t1);
                let t2 = &decryption(&buffer[..buffer_size], alpha, ra)[..];
                let message = String::from_utf8_lossy(t2);
                println!("\n{}\n{}", encrypted, message);
            },
            Err(error) => {
                println!("Connection error: {}", error.kind());
                break;
            }
        }
    }
}

fn writing(mut stream: TcpStream, receiver: mpsc::Receiver<String>, interruption: mpsc::Receiver<bool>, b: u64, rb: u64) {
    for message in receiver {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        let sent = encryption(message.as_bytes(), b, rb);
        if let Err(error) = stream.write(&sent[..]) {
            println!("Failed to send message due to: {}", error.kind());
            break;
        }
        let temp = without_decryption(&sent[..]);
        println!("{}", String::from_utf8_lossy(&temp[..]));
    }
}