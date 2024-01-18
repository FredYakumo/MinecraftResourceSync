// main.rs
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello(_req: HttpRequest) -> impl Responder {
    let addr = _req.peer_addr().expect("unknown addr");
    println!("Get connection from {addr}");
    return HttpResponse::Ok().body(format!("Your IP address: {}", addr.ip()));
}

#[post("/test")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Your ip:")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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