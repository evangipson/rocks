use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{builders::http_response::create_http_response, types::http_status::HttpStatus};

/// [`write_response_to_client`] will write an http response to
/// whatever client has connected to the server.
///
/// it also prints the ip address of the client that has connected
/// to the server.
fn write_response_to_client(mut stream: TcpStream) {
    println!(
        "stream connection from {}",
        stream
            .local_addr()
            .expect("could not find local address from stream.")
    );
    let _ = stream.write_all(
        create_http_response(HttpStatus::Ok, "welcome!")
            .as_bytes()
            .to_vec()
            .as_slice(),
    );
}

/// [`run_server`] creates a server which listens on the address
/// and port defined in `.cargo/config.toml`, then writes a response
/// using the private `write_response_to_client` method.
///
/// # example
/// ```rust
/// use rocks::services::server::run_server;
///
/// fn some_method() {
///     run_server();
/// }
/// ```
pub fn run_server() -> std::io::Result<()> {
    let env_ip_addr = env!("IP_ADDR", "IP_ADDR not defined in config.toml");
    let env_port = env!("PORT", "PORT not defined in config.toml");
    let listener = TcpListener::bind(format!("{env_ip_addr}:{env_port}"))?;

    // accept connections and process them serially
    println!("rocks is listening for requests at http://{env_ip_addr}:{env_port}");
    for stream in listener.incoming() {
        write_response_to_client(stream?);
    }
    Ok(())
}
