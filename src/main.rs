use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
    thread,
    time::Duration,
};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request= buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, contents) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string("hello.html").unwrap()),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string("hello.html").unwrap())
        }
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", fs::read_to_string("404.html").unwrap()),
    };

    let response = format!("{status_line}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
}
