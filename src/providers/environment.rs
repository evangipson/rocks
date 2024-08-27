use std::env;

pub fn get_server_endpoint() -> String {
    let env_ip_addr = env!("IP_ADDR", "IP_ADDR not defined in config.toml");
    let env_port = env!("PORT", "PORT not defined in config.toml");
    format!("{env_ip_addr}:{env_port}")
}
