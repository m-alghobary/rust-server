use std::sync::Arc;

use crate::http::{http_method::HttpMethod, request::Request, response::Response};

pub type RouteHandler = Arc<dyn Fn(Request) -> Response + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: RouteHandler,
}

impl Route {
    pub fn new(method: HttpMethod, path: String, handler: RouteHandler) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }

    pub fn get_params(&self) -> Vec<(usize, &str)> {
        let params: Vec<_> = self
            .path
            .split('/')
            .enumerate()
            .filter(|(_, part)| part.starts_with('{'))
            .collect();

        params
    }
}
