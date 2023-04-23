use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

mod request;
mod threadpool;

use crate::{
    request::{HttpMethod, Request},
    threadpool::ThreadPool,
};

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() {
    let listener = match TcpListener::bind(SERVER_ADDRESS) {
        Ok(listener) => listener,
        Err(_) => panic!("Failed to listen on {}!", SERVER_ADDRESS),
    };

    println!("Server is listening on {}", SERVER_ADDRESS);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection estaplished");

                match Request::try_from(&stream) {
                    Ok(request) => {
                        pool.execute(move || {
                            handle_connection(stream, request).unwrap();
                        });
                    }
                    Err(_) => {
                        eprintln!("Got Non-Http request");
                        continue;
                    }
                };
            }

            Err(_) => panic!("A failed connection!"),
        };
    }
}

fn handle_connection(mut stream: TcpStream, request: Request) -> std::io::Result<()> {
    println!("handle: {}", request.line.as_str());

    let (status_line, file) = match request.method {
        HttpMethod::Get => match request.path.as_str() {
            "/" => ("HTTP/1.1 200 OK", "index.html"),
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let body = std::fs::read_to_string(format!("static/{file}"))?;
    let length = body.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}
