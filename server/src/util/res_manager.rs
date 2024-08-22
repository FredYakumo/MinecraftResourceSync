use std::collections::{HashMap, HashSet};

use base::{models::Config, utils::res_loader::get_sha1_list_recursive};
use serde::{Deserialize, Serialize};
use base::models::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModClass {
    pub mod_sha1_file_map: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManagedMod {
    pub class_name_map: HashMap<String, ModClass>
}

impl ModClass {
    pub fn new(config: &Config) -> Result<ModClass> {
        let sha1_list = get_sha1_list_recursive(&config.manage_mod_file_path, "")?;
        Ok(ModClass {
            mod_sha1_file_map: HashMap::from_iter(
                sha1_list.iter().map(|v| (v.1.clone(), v.0.clone())),
            ),
        })
    }

    pub fn get_missing_list(&self, have_sha1_set: &HashSet<String>) -> Vec<(&String, &String)> {
        let mut ret = vec![];
        for (sha1, file_name) in self.mod_sha1_file_map.iter() {
            if !have_sha1_set.contains(sha1) {
                ret.push((sha1, file_name));
            }
        }
        ret
    }
}