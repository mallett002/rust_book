use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }

    // TODO: left off https://doc.rust-lang.org/book/ch21-01-single-threaded.html#reading-the-request
}
