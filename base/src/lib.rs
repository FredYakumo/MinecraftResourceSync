pub mod models;
pub mod utils;

use std::fs;
use std::io::Write;
use log::info;

use crate::models::Config;
use crate::utils::{res_loader, config_loader};
use crate::models::error::Result;

macro_rules! default_listen_port {
    () => {
        10233
    };
}

pub fn init(config_file_path: &str) -> Result<Config> {
    let config = match config_loader::get_config(config_file_path) {
        Ok(value) => value,
        Err(_err) => {
            let c = generate_default_config();
            config_loader::save_config_to_file(&c, config_file_path);
            info!("Config save to {}", config_file_path);
            c
        }
    };
    info!("Loading files from manage path...");
    info!("===================================");

    let sha1_list = res_loader::get_sha1_list_recursive(&config.manage_mod_file_path, "");
    for (file_name, sha1) in sha1_list? {
        info!("{}: {}", file_name, sha1);
    }
    info!("===================================");

    Ok(config)
}

fn generate_default_config() -> Config {
    print!("Manager mod file path: ");
    std::io::stdout().flush().unwrap();
    let mut manage_mod_file_path = String::new();
    std::io::stdin().read_line(&mut manage_mod_file_path).unwrap();
    manage_mod_file_path = String::from(manage_mod_file_path.trim());
    print!("Server listen port(default {}): ", default_listen_port!());
    std::io::stdout().flush().unwrap();
    let mut port_str = String::new();
    std::io::stdin().read_line(&mut port_str).unwrap();
    port_str = String::from(port_str.trim());
    
    Config {
        manage_mod_file_path,
        server_listen_port: if port_str.len() == 0 { default_listen_port!() } else { port_str.trim().parse::<u16>().unwrap()},
        mod_sync_class: None,
        server_url: None,
    }
}