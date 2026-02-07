use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr().unwrap());
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request:\n{:#?}", http_request);

    if http_request.is_empty() {
        return;
    }

    let request_line = &http_request[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 3 {
        return;
    }

    let method = parts[0];
    let path = parts[1];
    let _version = parts[2];

    if method != "GET" {
        let response = build_response(
            "HTTP/1.1 405 METHOD NOT ALLOWED",
            "Method Not Allowed",
            "text/plain",
        );
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let (status_line, body, content_type) = match path {
        "/" => match fs::read_to_string("static/index.html") {
            Ok(contents) => ("HTTP/1.1 200 OK", contents, "text/html"),
            Err(e) => {
                eprintln!("Error reading static/index.html: {}", e);
                (
                    "HTTP/1.1 500 INTERNAL SERVER ERROR",
                    "500 Internal Server Error".to_string(),
                    "text/plain",
                )
            }
        },
        "/hello" => (
            "HTTP/1.1 200 OK",
            "Hello from Rusty Web Server!".to_string(),
            "text/plain",
        ),
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            "404 Not Found".to_string(),
            "text/plain",
        ),
    };

    let response = build_response(status_line, &body, content_type);
    stream.write_all(response.as_bytes()).unwrap();
}

fn build_response(status_line: &str, body: &str, content_type: &str) -> String {
    format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content_type,
        body.len(),
        body
    )
}
