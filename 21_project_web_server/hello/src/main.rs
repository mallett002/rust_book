use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&stream);

    // read the request
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    // send response back to client
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // TODO: left off https://doc.rust-lang.org/book/ch21-01-single-threaded.html#validating-the-request-and-selectively-responding
}
