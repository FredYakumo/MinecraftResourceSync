mod utils;
mod module;
use utils::res_loader;
use module::server_module;

// main.rs
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

fn get_ip_info_from_request(_req: &HttpRequest) -> String {
    let addr = _req.peer_addr().expect("unknown addr");
    println!("Get connection from {addr}");
    return addr.ip().to_string();
}

#[get("/")]
async fn hello(_req: HttpRequest) -> impl Responder {
    let ip_addr = get_ip_info_from_request(&_req);
    return HttpResponse::Ok().body(format!("Your IP address: {}", ip_addr));
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match server_module::server_init() {
        Ok(value) => value,
        Err(err) => {
            panic!("Error loading init server: {}", err)
        }
    };

    let addrs = ("0.0.0.0", config.server_listen_port);
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(&addrs)?
    .run();

    println!("Server is listening on {}:{}", addrs.0, addrs.1);

    return server.await
}