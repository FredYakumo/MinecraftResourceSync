use serde:: { Serialize, Deserialize };

use super::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerRes {
    pub config: Config,
    pub file_sha1_list: Vec<(String, String)>
}