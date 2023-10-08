use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let response: String;

    let (status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let content_length = content.len();

    response = format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}