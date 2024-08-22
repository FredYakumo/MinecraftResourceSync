use std::sync::Arc;

use actix_web::{get, post, HttpRequest, HttpResponse, Responder};
use actix_web::web::Data;
use serde::Deserialize;

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

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ClassNameModHashList {
    pub className: String,
    pub modList: Vec<String>
}

#[post("/fetchMissingModList")]
pub async fn fetch_missiong_mod_list(class_name_mod_list: Data<Arc<Vec<ClassNameModHashList>>>) -> impl Responder {

}