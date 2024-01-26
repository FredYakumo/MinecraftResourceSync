use base::utils::config_loader::get_default_config_path;

pub fn client_run() {
    let config = base::init(format!("client_{}", get_default_config_path()).as_str())
        .unwrap();
}