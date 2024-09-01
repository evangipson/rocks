use super::server_endpoint::ServerEndpoint;

#[derive(Debug)]
pub struct ServerConfig {
    pub endpoints: Vec<ServerEndpoint>,
}
