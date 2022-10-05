use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("could not bind to tcp port");

    // incoming() returns an iterator that gives us a sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer_vec = vec![0; 1024];

    let http_response = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>();

    println!("{}", String::from_utf8_lossy(&buffer_vec));

    let repsonse = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(repsonse.as_bytes()).unwrap();
}
