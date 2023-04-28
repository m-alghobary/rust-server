mod app;
mod http;
mod server;
mod threadpool;

use app::App;
use http::{request::Request, response::Response};

use crate::server::Server;

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
    Server::new(
        App::new()
            .get("/", |_request: Request| -> Response {
                Response::ok_from_file("static/index.html").unwrap()
            })
            .get("/about", |request: Request| -> Response {
                let name: String = request.get_query_param("name").unwrap_or("Ali".to_owned());
                Response::ok(format!("Hi {}", name).as_str())
            })
            .get("/users/{id}", |request: Request| -> Response {
                let id: u32 = request.get_route_param("id").unwrap_or(1);
                Response::ok(format!("Hi user => {}", id).as_str())
            }),
    )
    .listen(SERVER_ADDRESS)?
    .run()?;

    Ok(())
}
