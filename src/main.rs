mod request;
mod response;
mod server;
mod threadpool;

use crate::server::Server;

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.listen(SERVER_ADDRESS)?;

    Ok(())
}
