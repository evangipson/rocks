use std::net::{SocketAddr, UdpSocket};

use crate::{contracts::rocks_server::RocksServer, providers::environment::get_server_endpoint};

impl RocksServer<UdpSocket> for UdpSocket {
    fn write_response_to_client(&mut self) {
        panic!("cannot write response to client with UDP.");
    }

    /// [`write_response_to_client`] will write an http response to
    /// whatever client has connected to the server.
    ///
    /// it also prints the ip address of the client that has connected
    /// to the server.
    fn write_response_to_socket(&mut self, address: SocketAddr) {
        println!(
            "stream connection from {}",
            &self
                .local_addr()
                .expect("could not find local address from stream.")
        );
        let _ = &self.send_to("welcome!".as_bytes().to_vec().as_slice(), address);
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
        let mut socket = UdpSocket::bind(server_listener_endpoint).unwrap_or_else(|_| {
            panic!("could not bind to server listener endpoint \"{server_listener_endpoint}\"")
        });

        // accept connections and process them serially
        println!("rocks is listening for requests at http://{server_listener_endpoint}");

        let mut buf = [0; 50];
        let (_, src_addr) = socket.recv_from(&mut buf).unwrap();
        println!("src_addr: {src_addr}");
        socket.write_response_to_socket(src_addr);
        // let filled_buf = &mut buf[..number_of_bytes];
        // println!("filled_buf: {:?}", filled_buf);
        Ok(())
    }
}
