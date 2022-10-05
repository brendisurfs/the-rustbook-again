use c_20_server::ThreadPool;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    // creating a finite number of threads
    let pool = ThreadPool::new(6);

    let listener = TcpListener::bind("127.0.0.1:7878").expect("could not bind to tcp port");
    println!("started server on 7878");

    // incoming() returns an iterator that gives us a sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("request line: {}", request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET /HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }

        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let file_len = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {file_len}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
