use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;


macro_rules! default_config_file_name {
    () => {
        "config.yaml"
    };
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub manage_mod_file_path: String,
}

pub fn get_config() -> Config {
    let file_path = default_config_file_name!();

    let mut file = File::open(file_path).expect(format!("Unable to open file: {}", file_path).as_str());

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Unable to read file: {}", file_path).as_str());

    let ci: Config = serde_yaml::from_str(&contents).expect("Unable to parse YAML");

    println!("{:?}", ci);

    return ci;
}