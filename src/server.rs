#![allow(dead_code)]

use std::{io::Write, net::TcpListener};

use crate::{
    app::App,
    http::{request::Request, response::Response},
    threadpool::ThreadPool,
};

/// The number of worker threads the server has in its pool of threads
const THREAD_POOL_SIZE: usize = 4;

pub struct Server {
    /// The address (IP:PORT) this server is bound to
    address: String,

    /// a pool of worker threads the server uses to handle incoming request
    thread_pool: ThreadPool,

    /// the main application for request handling
    app: App,

    /// if we fail to bind to the supplied address this will be None
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(app: App) -> Self {
        let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);

        Self {
            address: String::new(),
            thread_pool,
            app,
            listener: None,
        }
    }

    /// Start listening on the provided address
    ///
    /// It return std::io::Error if it can not listen on the provided address for any reasoun.
    ///
    pub fn listen(mut self, address: &str) -> std::io::Result<Self> {
        self.address = address.to_owned();

        let bind_result = TcpListener::bind(address);
        if let Ok(listener) = bind_result {
            self.listener = Some(listener);
        } else {
            return Err(bind_result.unwrap_err());
        }

        Ok(self)
    }

    ///
    /// runs the app and start accepting connections
    ///  
    pub fn run(&mut self) -> std::io::Result<()> {
        println!("Server is listening on {}", self.address);

        for stream in self.listener.as_mut().unwrap().incoming() {
            println!("Connection estaplished");

            let mut stream = stream?;

            // try to read request data from the TcpStream and construct a basic HTTP Request object from it
            // this will fail if the request was not an HTTP Request
            match Request::initial_parse(&stream) {
                Ok(mut request) => {
                    // if we got an HTTP Request,
                    //
                    // 1. we try to find any registered handler that matches the request method and path
                    match self.app.get_route(request.method, &request.base_path) {
                        // 2. if we found one we dispatch a job useing the thread pool to execute the handler
                        Some(route) => {
                            // after we get a matching route object we can now continue parsing the whole request object
                            match request.complete_parsing(&stream, &route) {
                                Ok(_) => {}
                                Err(_) => {
                                    eprintln!("Faild to complete parsing request {:?}", request);
                                    continue;
                                }
                            }

                            self.thread_pool.execute(move || {
                                let response = (route.handler)(request);
                                stream.write_all(response.as_string().as_bytes()).unwrap();
                            });
                        }

                        // 3. if we did not found any handler we return NOT FOUND error
                        None => {
                            self.thread_pool.execute(move || {
                                let response = Response::not_found();
                                stream.write_all(response.as_string().as_bytes()).unwrap();
                            });
                        }
                    };
                }
                Err(_) => {
                    eprintln!("Got Non-Http request");
                    continue;
                }
            };
        }

        Ok(())
    }
}
