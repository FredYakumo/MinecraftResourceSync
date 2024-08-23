mod config;
pub mod error;
pub mod request;

use std::collections::HashMap;

pub use config::Config;

use serde:: { Serialize, Deserialize };

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ModClass {
    pub class_name: String,
    /**
     *  Key: file_hash, Value file_name
    */
    pub mod_list: HashMap<String, String>
}