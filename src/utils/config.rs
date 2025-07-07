use serde::{Deserialize, Serialize};
use static_init::dynamic;

#[dynamic]
pub static APP_CONFIG: AppConfig = {
    let mut file = std::fs::File::open("config.yaml").unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    serde_yaml::from_str(&contents).unwrap()
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppConfig {
    pub masternode_ips: Vec<String>,
    pub app_port: usize,
}
