use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Incoming request from {}", stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let handled_get = b"GET / HTTP/1.1\r\n";

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status, html_filename) = if buffer.starts_with(handled_get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let html = fs::read_to_string(html_filename).unwrap();
    let response = format!("{}{}", status, html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
