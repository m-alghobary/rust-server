mod request;
mod response;
mod route;
mod server;
mod threadpool;

use request::Request;
use response::Response;

use crate::server::Server;

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
    let mut server = Server::new();

    server.get("/", |_request: Request| -> Response {
        Response::ok_from_file("static/index.html").unwrap()
    });

    server.get("/about", |_request: Request| -> Response {
        Response::ok("About")
    });

    server.listen(SERVER_ADDRESS)?;

    Ok(())
}
