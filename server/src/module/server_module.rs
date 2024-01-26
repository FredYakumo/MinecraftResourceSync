use actix_web::{get, HttpResponse, Responder, HttpRequest};

fn get_ip_info_from_request(_req: &HttpRequest) -> String {
    let addr = _req.peer_addr().expect("unknown addr");
    println!("Get connection from {addr}");
    return addr.ip().to_string();
}

#[get("/")]
pub async fn req_process(_req: HttpRequest) -> impl Responder {
    let ip_addr = get_ip_info_from_request(&_req);
    return HttpResponse::Ok().body(format!("Your IP address: {}", ip_addr));
}