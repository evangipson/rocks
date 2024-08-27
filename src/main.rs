use rocks::contracts::rocks_server::RocksServer;
use std::net::TcpStream;

/// [`main`] is the entry point for [`rocks`].
///
/// it will use [`run_server`] to begin the server.
fn main() {
    match TcpStream::run_server() {
        Ok(()) => println!("server started"),
        Err(_) => println!("server failed to start"),
    }
}
