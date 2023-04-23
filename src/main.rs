use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use server::ThreadPool;

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
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

                pool.execute(|| {
                    handle_connection(stream).unwrap();
                });
            }

            Err(_) => panic!("A failed connection!"),
        };
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(&mut stream);

    // let request: Vec<_> = reader
    //     .lines()
    //     .filter_map(|line| line.ok())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", request);

    let request_line = reader.lines().next().unwrap()?;

    let (status_line, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let body = std::fs::read_to_string(format!("static/{file}"))?;
    let length = body.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}
