#![allow(dead_code)]

use std::{collections::HashMap, io::Write, net::TcpListener, sync::Arc};

use crate::{
    http::{http_method::HttpMethod, request::Request, response::Response},
    route::Route,
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
    ///
    pub fn listen(&mut self, address: &str) -> std::io::Result<()> {
        self.address = address.to_owned();
        let listener = TcpListener::bind(address)?;

        println!("Server is listening on {}", address);

        for stream in listener.incoming() {
            println!("Connection estaplished");

            let mut stream = stream?;

            // try to read request data from the TcpStream and construct an HTTP Request object from it
            // this will fail if the request was not an HTTP Request
            match Request::parse(&stream) {
                Ok(request) => {
                    // if we got an HTTP Request,
                    //
                    // 1. we try to find any registered handler that matches the request method and path
                    match self.get_route(request.method, &request.path) {
                        // 2. if we found one we dispatch a job useing the thread pool to execute the handler
                        Some(route) => {
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

    /// Register an HTTP GET route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Get, path, handler);
    }

    /// Register an HTTP POST route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Post, path, handler);
    }

    /// Register an HTTP PUT route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Put, path, handler);
    }

    /// Register an HTTP DELETE route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Delete, path, handler);
    }

    fn register_route<F>(&mut self, method: HttpMethod, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        let method_routes = self.routes.entry(method).or_insert(Vec::new());

        let path_exist = method_routes.iter().any(|route| route.path == path);
        if path_exist {
            panic!("this `{:?} {}` path is already registered!", method, path);
        }

        // register the route
        method_routes.push(Route::new(method, path.to_owned(), Arc::new(handler)));
    }

    fn get_route(&mut self, method: HttpMethod, path: &String) -> Option<Route> {
        let method_routes = self.routes.entry(method).or_insert(Vec::new());

        method_routes
            .iter()
            .find(|route| {
                // if the two paths do not have the same number of slashes '/'
                // this means they do not match
                if route.path.split('/').count() != path.split('/').count() {
                    return false;
                }

                // get the route params as (index, {name})
                let route_params = route.get_params();
                if route_params.is_empty() {
                    // no params defined so we jsut compare the two paths as strings
                    &route.path == path
                } else {
                    // here we know both paths have the same length (or slashes)
                    // and we know the names and positions of the defined params,
                    // so the paths will be considered a match if they have the same non-param sluts

                    let param_indexs: Vec<_> = route_params.iter().map(|param| param.0).collect();

                    route
                        .path
                        .split('/')
                        .zip(path.split('/'))
                        .enumerate()
                        .all(|(i, (f, s))| {
                            if !param_indexs.contains(&i) {
                                return f == s;
                            }

                            true
                        })
                }
            })
            .map(|route| route.to_owned())
    }
}
