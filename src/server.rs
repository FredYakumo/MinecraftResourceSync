mod utils;

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

#[get("/test")]
async fn echo(_req: HttpRequest) -> impl Responder {
    println!("On test request!");
    return get_ip_info_from_request(&_req);
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Your ip:")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::config_loader::get_config();


    let addrs = ("0.0.0.0", 10233);
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(&addrs)?
    .run();

    println!("Server is listening on {}:{}", addrs.0, addrs.1);

    return server.await
}