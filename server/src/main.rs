use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    let listenip = "127.0.0.1";
    let listenport = "8080";
    let listener = TcpListener::bind(format!("{}:{}", listenip, listenport)).unwrap();
    
    // Client ip
    match listener.accept() {
        Ok((_socket, addr)) => println!("New client: {:#?}", addr),
        Err(e) => println!("Couldn't get client: {:#?}", e),
    }
    
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).unwrap();
    
    let post = b"POST / HTTP/1.1\r\n";
    
    let get = b"GET / HTTP/1.1\r\n";
    
    if buffer.starts_with(get) {
    
        let contents = fs::read_to_string("html.html").unwrap();
        
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else if buffer.starts_with(post) {
        let contents = fs::read_to_string("post.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
    
    
    
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
