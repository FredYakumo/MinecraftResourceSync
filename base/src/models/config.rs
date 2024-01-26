use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub manage_mod_file_path: String,
    pub server_listen_port: u16,
}