mod config;

pub use config::Config;

use serde:: { Serialize, Deserialize };


#[derive(Debug, Serialize, Deserialize)]
pub struct ModSyncRequest {
    pub mod_sha1_list: Vec<(String, String)>,
    pub mod_sync_class: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerRes {
    pub config: Config,
    pub file_sha1_list: Vec<(String, String)>
}