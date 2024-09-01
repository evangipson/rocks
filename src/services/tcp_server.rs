use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    builders::http_response::create_http_response,
    providers::environment::get_server_address,
    types::{http_method::HttpMethod, http_status::HttpStatus, server_endpoint::ServerEndpoint},
};

fn read_tcp_stream(mut stream: &TcpStream) -> String {
    let mut buffer = Vec::new();
    const CHUNK_SIZE: usize = 256;
    loop {
        let mut chunk = [0; CHUNK_SIZE];
        let bytes_read = stream
            .read(&mut chunk)
            .expect("could not read chunk of tcp stream to buffer");
        if bytes_read == 0 {
            // End of stream reached
            break;
        }
        // Resize buffer if needed
        if buffer.len() + bytes_read > buffer.capacity() {
            let new_capacity = buffer.capacity() * 2;
            buffer.reserve(new_capacity - buffer.len());
        }
        buffer.extend_from_slice(&chunk[..bytes_read]);
        if bytes_read < CHUNK_SIZE {
            // End of stream reached
            break;
        }
    }
    std::str::from_utf8(&buffer)
        .expect("could not read tcp stream as utf8")
        .to_string()
}

/// [`write_response_to_client`] will write an http response to
/// whatever client has connected to the server.
///
/// it also prints the ip address of the client that has connected
/// to the server.
fn write_response_to_client(mut tcp_stream: &TcpStream, response_message: String) {
    println!(
        "stream connection from {}",
        tcp_stream
            .local_addr()
            .expect("could not find local address from stream.")
    );
    let _ = tcp_stream.write_all(
        create_http_response(HttpStatus::Ok, &response_message)
            .as_bytes()
            .to_vec()
            .as_slice(),
    );
}

fn get_router_response(tcp_stream: TcpStream, server_endpoints: &[ServerEndpoint]) {
    let parsed_tcp_stream = read_tcp_stream(&tcp_stream);
    let request_line = parsed_tcp_stream
        .lines()
        .next()
        .expect("did not find a new line in the tcp stream, more than likely an invalid request.");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = if parts[0] == "GET" {
        HttpMethod::GET
    } else {
        HttpMethod::POST
    };
    let path = parts[1];

    // Check if the endpoint matches any defined endpoint
    let found_endpoint = server_endpoints
        .iter()
        .find(|endpoint| endpoint.method == method && endpoint.path == path);

    // If an endpoint is found, write the corresponding response
    if let Some(endpoint) = found_endpoint {
        write_response_to_client(&tcp_stream, endpoint.response.to_string());
    } else {
        // If no endpoint is found, handle the default case ("/" or "")
        match (method, path) {
            (HttpMethod::GET, "") | (HttpMethod::GET, "/") => {
                write_response_to_client(&tcp_stream, "Index page".to_string())
            }
            _ => write_response_to_client(&tcp_stream, "error!".to_string()),
        }
    }
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
pub fn run_server(server_endpoints: Vec<ServerEndpoint>) -> std::io::Result<()> {
    let server_listener_endpoint = &get_server_address();
    let listener = TcpListener::bind(server_listener_endpoint).unwrap_or_else(|_| {
        panic!("could not bind to server listener endpoint \"{server_listener_endpoint}\"")
    });

    // accept connections and process them serially
    println!("rocks is listening for requests at http://{server_listener_endpoint}");
    for stream in listener.incoming() {
        //thread::spawn(|| {
        let tcp_stream = stream.unwrap();
        get_router_response(tcp_stream, &server_endpoints);
        //});
    }
    Ok(())
}
