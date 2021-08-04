use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// what's mut? wtf does the ':' mean? - 0 exp rust programmer
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
                        // what is this? an array?
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTPS/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}