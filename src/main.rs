const SERVER_ADDRESS: &str = "localhost:7070";

use rs_server::app::App;
use rs_server::http::request::Request;
use rs_server::http::response::Response;
use rs_server::server::Server;

fn main() -> std::io::Result<()> {
    Server::new(
        App::default()
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
