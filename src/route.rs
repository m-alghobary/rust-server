use crate::{
    request::{HttpMethod, Request},
    response::Response,
};

pub type RouteHandler = Box<dyn FnOnce(Request) -> Response + Send + 'static>;

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
}
