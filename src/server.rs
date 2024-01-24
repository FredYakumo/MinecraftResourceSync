mod utils;
mod module;

use module::base;
use module::server_module;

// main.rs
use actix_web::{App, HttpServer, HttpRequest};

fn get_ip_info_from_request(_req: &HttpRequest) -> String {
    let addr = _req.peer_addr().expect("unknown addr");
    println!("Get connection from {addr}");
    return addr.ip().to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match base::load_general_config() {
        Ok(value) => value,
        Err(err) => {
            panic!("Error loading init server: {}", err)
        }
    };

    let addrs = ("0.0.0.0", config.server_listen_port);
    let server = HttpServer::new(|| {
        App::new()
            .service(server_module::req_process)
    })
    .workers(4)
    .bind(&addrs)?
    .run();

    println!("Server is listening on {}:{}", addrs.0, addrs.1);

    return server.await
}