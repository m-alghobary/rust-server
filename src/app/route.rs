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

    ///
    /// Get defined route params if any.
    /// It returns each param is a tuple in the form of (index, name).
    ///
    /// `index` is the index of the route segment in which the param appears.
    /// `name` is the name of the param.
    ///
    pub fn get_params(&self) -> Vec<(usize, &str)> {
        let params: Vec<_> = self
            .path
            .split('/')
            .enumerate()
            .filter(|(_, part)| part.starts_with('{'))
            .map(|(i, name)| (i, &name[1..name.len() - 1]))
            .collect();

        params
    }
}

/// Unit Tests
#[cfg(test)]
mod tests {

    use super::*;

    fn init_route(path: &str) -> Route {
        let handler: RouteHandler = Arc::new(|_r: Request| -> Response { todo!() });
        Route::new(HttpMethod::Get, path.to_owned(), handler)
    }

    #[test]
    fn get_params_work_with_no_param() {
        let path = "/posts/all";
        let route = init_route(path);

        let params = route.get_params();

        assert_eq!(params.len(), 0);
    }

    #[test]
    fn get_params_work_with_one_param() {
        let path = "/post/{id}/comments";
        let route = init_route(path);

        let params = route.get_params();

        assert_eq!(params.len(), 1);
        assert_eq!(params[0], (2, "id"));
    }

    #[test]
    fn get_params_work_with_just_param() {
        let path = "/{id}";
        let route = init_route(path);

        let params = route.get_params();

        assert_eq!(params.len(), 1);
        assert_eq!(params[0], (1, "id"));
    }

    #[test]
    fn get_params_work_with_more_than_one_param() {
        let path = "/post/{id}/comments/{comment_id}";
        let route = init_route(path);

        let params = route.get_params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0], (2, "id"));
        assert_eq!(params[1], (4, "comment_id"));
    }
}
