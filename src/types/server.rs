use super::server_endpoint::ServerEndpoint;

/// [`Server`] is filled with endpoints that are
/// supported by [`rocks`].
pub struct Server {
    pub server_endpoints: Vec<ServerEndpoint>,
}
