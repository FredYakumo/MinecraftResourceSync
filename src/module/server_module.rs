use actix_web::{get, HttpResponse, Responder, HttpRequest};

use crate::get_ip_info_from_request;

use super::super::utils::{res_loader, config_loader, config_loader::Config};

#[get("/")]
pub async fn req_process(_req: HttpRequest) -> impl Responder {
    let ip_addr = get_ip_info_from_request(&_req);
    return HttpResponse::Ok().body(format!("Your IP address: {}", ip_addr));
}