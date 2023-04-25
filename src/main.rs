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

    server.get("/", |request: Request| -> Response { todo!() });

    server.get("/about", |request: Request| -> Response { todo!() });

    server.listen(SERVER_ADDRESS)?;

    Ok(())
}
