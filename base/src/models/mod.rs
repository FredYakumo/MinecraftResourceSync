mod config;
pub mod error;
pub mod request;

use std::collections::HashMap;
use crate::models::error::Result;

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

impl ModClass {
    pub fn new(config: &Config) -> Result<ModClass> {
        let sha1_list = get_sha1_list_recursive(&config.manage_mod_file_path, "")?;
        Ok(ModClass {
            mod_list: HashMap::from_iter(
                sha1_list.iter().map(|v| (v.1.clone(), v.0.clone())),
            ),
        })
    }

    pub fn get_missing_sha1_file_name_map(&self, have_sha1_set: &HashSet<String>) -> HashMap<String, String> {
        let mut ret = hashmap!{};
        for (sha1, file_name) in self.mod_sha1_file_map.iter() {
            if !have_sha1_set.contains(sha1) {
                ret.insert(sha1.clone(), file_name.clone());
            }
        }
        ret
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManagedMod {
    pub class_name_map: HashMap<String, ModClass>
}