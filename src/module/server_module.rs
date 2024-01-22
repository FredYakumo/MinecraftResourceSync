use std::io::Write;

use actix_web::{get, HttpResponse, Responder, HttpRequest};

use crate::get_ip_info_from_request;

use super::super::utils::{res_loader, config_loader, config_loader::Config};

macro_rules! default_listen_port {
    () => {
        10233
    };
}

pub fn server_init() -> Result<Config, Box<dyn std::error::Error>> {
    let config = match config_loader::get_config() {
        Ok(value) => value,
        Err(_err) => { 
            let c = generate_default_config();
            config_loader::save_config_to_file(&c);
            println!("Config save to {}", config_loader::get_config_file_path());
            c
        }
    };
    println!("Loading files from manage path...");
    println!("===================================");
    let sha1_list = res_loader::get_sha1_list_recursive(&config.manage_mod_file_path, "");
    for (file_name, sha1) in sha1_list? {
        println!("{}: {}", file_name, sha1);
    }
    println!("===================================");

    Ok(config)
}

#[get("/")]
pub async fn req_process(_req: HttpRequest) -> impl Responder {
    let ip_addr = get_ip_info_from_request(&_req);
    return HttpResponse::Ok().body(format!("Your IP address: {}", ip_addr));
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
        server_listen_port: if port_str.len() == 0 { default_listen_port!() } else { port_str.trim().parse::<u16>().unwrap()}
    }
}