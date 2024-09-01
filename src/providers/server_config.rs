use std::fs;
use toml::Table;

use crate::providers::environment::get_working_directory;
use crate::types::{server_config::ServerConfig, server_endpoint::ServerEndpoint};

pub fn get_server_config() -> ServerConfig {
    let working_directory = get_working_directory();
    let server_config_file_contents =
        fs::read_to_string(format!("{}/.cargo/server-config.toml", working_directory))
            .expect("could not find server-config.toml file.");
    let server_config_table = &server_config_file_contents.parse::<Table>().unwrap();
    let server_endpoints_table = server_config_table["endpoints"].as_table().unwrap();
    let endpoints: Vec<ServerEndpoint> = server_endpoints_table
        .iter()
        .map(|x| {
            let server_endpoint_test = x.1.as_array().unwrap();
            let server_map = server_endpoint_test[0].as_table().unwrap();
            ServerEndpoint {
                method: <toml::Value as Clone>::clone(&server_map["method"])
                    .try_into()
                    .unwrap(),
                path: <toml::Value as Clone>::clone(&server_map["path"])
                    .try_into()
                    .unwrap(),
                response: <toml::Value as Clone>::clone(&server_map["response"])
                    .try_into()
                    .unwrap(),
            }
        })
        .collect();
    ServerConfig { endpoints }
}
