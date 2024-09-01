extern crate rocks;
use rocks::{providers::server_config::get_server_config, services::tcp_server::run_server};

/// [`main`] is the entry point for [`rocks`].
///
/// it will use [`run_server`] to begin the server.
fn main() {
    let server_config = get_server_config();
    let _ = run_server(server_config.endpoints);
}
