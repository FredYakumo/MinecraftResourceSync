use std::collections::HashMap;

use base::{models::{Config, ModSyncRequest}, utils::{config_loader::get_default_config_path, res_loader::get_sha1_list_recursive}};
use reqwest::Client;

fn get_request(config: &Config) -> ModSyncRequest {
    ModSyncRequest {
        mod_sync_class: config.mod_sync_class,
        mod_sha1_list: get_sha1_list_recursive(config.manage_mod_file_path.as_str(), "")
    }
}

async fn do_http_sync_request(config: &Config) {
    let request = get_request(&config);
    let client = Client::new();
    let response = client.post(format!("{}:{}", config.server_url, config.server_listen_port))
        .json(&json!(request))
        .send()
        .await?;
    dbg!(response.json()?);
}

pub async fn client_run() {
    let config = base::init(format!("client_{}", get_default_config_path()).as_str())
        .unwrap();
    do_http_sync_request(&config).await;
}