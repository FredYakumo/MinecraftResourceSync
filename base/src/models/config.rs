use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub manage_mod_file_path: String,
    pub server_listen_port: u16,
    pub mod_sync_class: Option<String>,
    pub server_url: Option<String>
}