mod service;
mod util;

use std::{collections::HashMap, sync::Arc};

use base::{
    init,
    utils::{config_loader::get_default_config_path, res_loader::get_sha1_list_recursive},
};
use log::{error, info};
use log_util::log_util::LogUtil;
use maplit::hashmap;
// main.rs
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use service::fetch_missing_mod;
use util::res_manager::{ManagedMod, ModClass};

lazy_static! {
    static ref LOG: LogUtil = LogUtil::new("server");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match base::init(format!("server_{}", get_default_config_path()).as_str()) {
        Ok(config) => config,
        Err(err) => {
            error!("Error: {}", err);
            return Err(err.into());
        }
    };

    let default_class = ModClass::new(&config)?;
    let managed_mod = Arc::new(ManagedMod {
        class_name_map: hashmap! {
            "default".to_string() => default_class
        },
    });

    let addrs = ("0.0.0.0", config.server_listen_port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 1024 * 8))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(managed_mod.clone()))
            .service(fetch_missing_mod::req_process)
    })
    .workers(4)
    .bind(&addrs)?
    .run();

    info!("Server is listening on {}:{}", addrs.0, addrs.1);

    return server.await;
}
