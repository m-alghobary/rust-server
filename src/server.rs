use std::{collections::HashMap, io::Write, net::TcpListener};

use crate::{
    request::{HttpMethod, Request},
    response::Response,
    route::{Route, RouteHandler},
    threadpool::ThreadPool,
};

/// The number of worker threads the server has in its pool of threads
const THREAD_POOL_SIZE: usize = 4;

pub struct Server {
    /// The address (IP:PORT) this server is bound to
    address: String,

    /// a pool of worker threads the server uses to handle incoming request
    thread_pool: ThreadPool,

    /// the list of registered routes
    routes: HashMap<HttpMethod, Vec<Route>>,
}

impl Server {
    pub fn new() -> Self {
        let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);

        Self {
            address: String::new(),
            thread_pool,
            routes: HashMap::new(),
        }
    }

    /// Start listening on the provided address
    ///
    /// It return std::io::Error if it can not listen on the provided address for any reasoun.
    /// Else it will start listening for connections
    pub fn listen(&mut self, address: &str) -> std::io::Result<()> {
        self.address = address.to_owned();
        let listener = TcpListener::bind(address)?;

        println!("Server is listening on {}", address);

        for stream in listener.incoming() {
            println!("Connection estaplished");

            let mut stream = stream?;

            // try to read request data from the TcpStream and construct an HTTP Request object from it
            // this will fail if the request was not an HTTP Request
            match Request::try_from(&stream) {
                Ok(_request) => {
                    // if we got an HTTP Request,
                    //
                    // TODO: Missing things to do
                    // 1. we try to find any registered handler that matches the request method and path
                    // 2. if we found one we dispatch a job useing the thread pool to execute the handler
                    // 3. if we did not found any handler we return NOT FOUND error

                    self.thread_pool.execute(move || {
                        let response = Response::ok_from_file("static/index.html").unwrap();
                        stream.write_all(response.as_string().as_bytes()).unwrap();
                    });
                }
                Err(_) => {
                    eprintln!("Got Non-Http request");
                    continue;
                }
            };
        }

        Ok(())
    }

    /// Register an HTTP GET route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: FnOnce(Request) -> Response + Send + 'static,
    {
        let get_routes = self.routes.entry(HttpMethod::Get).or_insert(Vec::new());

        let path_exist = get_routes.iter().any(|route| route.path == path);
        if path_exist {
            panic!("this `GET {}` path is already registered!", path);
        }

        // register the route
        get_routes.push(Route::new(
            HttpMethod::Get,
            path.to_owned(),
            Box::new(handler),
        ));
    }
}
