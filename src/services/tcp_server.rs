use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use crate::{
    builders::http_response::create_http_response, contracts::rocks_server::RocksServer,
    providers::environment::get_server_endpoint, types::http_status::HttpStatus,
};

impl RocksServer<TcpStream> for TcpStream {
    /// [`write_response_to_client`] will write an http response to
    /// whatever client has connected to the server.
    ///
    /// it also prints the ip address of the client that has connected
    /// to the server.
    fn write_response_to_client(&mut self) {
        println!(
            "stream connection from {}",
            &self
                .local_addr()
                .expect("could not find local address from stream.")
        );
        let _ = &self.write_all(
            create_http_response(HttpStatus::Ok, "welcome!")
                .as_bytes()
                .to_vec()
                .as_slice(),
        );
    }

    fn write_response_to_socket(&mut self, _address: SocketAddr) {
        panic!("cannot write tcp sockets");
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
    fn run_server() -> std::io::Result<()> {
        let server_listener_endpoint = &get_server_endpoint();
        let listener = TcpListener::bind(server_listener_endpoint).unwrap_or_else(|_| {
            panic!("could not bind to server listener endpoint \"{server_listener_endpoint}\"")
        });

        // accept connections and process them serially
        println!("rocks is listening for requests at http://{server_listener_endpoint}");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            thread::spawn(move || {
                stream.write_response_to_client();
            });
        }
        Ok(())
    }
}
