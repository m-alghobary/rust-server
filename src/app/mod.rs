#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use crate::http::{http_method::HttpMethod, request::Request, response::Response};

use self::route::Route;

pub mod route;

pub struct App {
    /// the list of registered routes
    routes: HashMap<HttpMethod, Vec<Route>>,
}

impl App {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register an HTTP GET route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Get, path, handler);
        self
    }

    /// Register an HTTP POST route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn post<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Post, path, handler);
        self
    }

    /// Register an HTTP PUT route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn put<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Put, path, handler);
        self
    }

    /// Register an HTTP DELETE route handler
    ///
    /// # Panic
    /// this method will panic if the path is already registered
    ///
    pub fn delete<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.register_route(HttpMethod::Delete, path, handler);
        self
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

    pub fn get_route(&mut self, method: HttpMethod, path: &String) -> Option<Route> {
        let method_routes = self.routes.entry(method).or_insert(Vec::new());

        method_routes
            .iter()
            .find(|route| {
                // if the two paths do not have the same number of route segments
                // this means they do not match
                if route.path.split('/').count() != path.split('/').count() {
                    return false;
                }

                // get the route params as (index, name)
                let route_params = route.get_params();
                if route_params.is_empty() {
                    // no params defined so we jsut compare the two paths as strings
                    &route.path == path
                } else {
                    // here we know both paths have the same number of segments
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
