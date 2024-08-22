mod module;

use base::{ init, utils::config_loader::get_default_config_path };
use log::{error, info};
// main.rs
use module::server_module;
use actix_web::{App, HttpServer};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: LogUtil = LogUtil::new("server");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = base::init(format!("server_{}", get_default_config_path()).as_str())?;

    let addrs = ("0.0.0.0", config.server_listen_port);
    let server = HttpServer::new(|| {
        App::new()
            .service(server_module::req_process)
    })
    .workers(4)
    .bind(&addrs)?
    .run();

    info!("Server is listening on {}:{}", addrs.0, addrs.1);

    return server.await
}