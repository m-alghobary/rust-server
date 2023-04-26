mod http;
mod route;
mod server;
mod threadpool;

use http::{request::Request, response::Response};

use crate::server::Server;

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
    let mut server = Server::new();

    server.get("/", |_request: Request| -> Response {
        Response::ok_from_file("static/index.html").unwrap()
    });

    server.get("/about", |request: Request| -> Response {
        let name: String = request.get_query_param("name").unwrap_or("Ali".to_owned());
        Response::ok(format!("Hi {}", name).as_str())
    });

    server.listen(SERVER_ADDRESS)?;

    Ok(())
}
