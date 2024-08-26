use rocks::services::server::run_server;

/// [`main`] is the entry point for [`rocks`].
///
/// it will use [`run_server`] to begin the server.
fn main() {
    match run_server() {
        Ok(()) => println!("server started"),
        Err(_) => println!("server failed to start"),
    }
}
