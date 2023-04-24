use std::{io::Write, net::TcpListener};

mod request;
mod response;
mod threadpool;

use crate::{
    request::{HttpMethod, Request},
    response::Response,
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
            Ok(mut stream) => {
                println!("Connection estaplished");

                match Request::try_from(&stream) {
                    Ok(request) => {
                        pool.execute(move || {
                            let response = handle_connection(request).unwrap();
                            stream.write_all(response.as_string().as_bytes()).unwrap();
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

fn handle_connection(request: Request) -> std::io::Result<Response> {
    println!("handle: {}", request.line.as_str());

    match request.method {
        HttpMethod::Get => match request.path.as_str() {
            "/" => Response::ok_from_file("static/index.html"),
            _ => Response::not_found_from_file("static/404.html"),
        },
        _ => Response::not_found_from_file("static/404.html"),
    }
}
