use log::{debug, info};

use crate::models::Config;
use std::fs::File;
use std::io::prelude::*;

macro_rules! default_config_file_name {
    () => {
        "config.yaml"
    };
}

pub fn get_default_config_path() -> &'static str {
    return default_config_file_name!();
}

pub fn get_config(config_file_path: &str) -> std::io::Result<Config> {
    let mut file =
        File::open(config_file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Unable to read file: {}", config_file_path).as_str());

    let ci: Config = serde_yaml::from_str(&contents).expect("Unable to parse YAML");

    debug!("{:?}", ci);

    Ok(ci)
}

pub fn save_config_to_file(config: &Config, config_file_path: &str) {
    let mut file = File::create(config_file_path).expect(format!("Write file to {} failed", config_file_path).as_str());
    file.write_all(serde_yaml::to_string(config).unwrap().as_bytes()).unwrap();
}
