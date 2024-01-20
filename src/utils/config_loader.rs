use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

macro_rules! default_config_file_name {
    () => {
        "config.yaml"
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub manage_mod_file_path: String,
    pub server_listen_port: u16,
}

pub fn get_config_file_path() -> &'static str {
    default_config_file_name!()
}

pub fn get_config() -> std::io::Result<Config> {
    let file_path = default_config_file_name!();

    let mut file =
        File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Unable to read file: {}", file_path).as_str());

    let ci: Config = serde_yaml::from_str(&contents).expect("Unable to parse YAML");

    println!("{:?}", ci);

    Ok(ci)
}

pub fn save_config_to_file(config: &Config) {
    let mut file = File::create(default_config_file_name!()).unwrap();
    file.write_all(serde_yaml::to_string(config).unwrap().as_bytes()).unwrap();
}
