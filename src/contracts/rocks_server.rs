use std::net::SocketAddr;

pub trait RocksServer<TServerType> {
    fn write_response_to_client(&mut self);
    fn write_response_to_socket(&mut self, address: SocketAddr);
    fn run_server() -> std::io::Result<()>;
}
