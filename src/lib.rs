//! # rocks
//! rocks is a **r**ust d**ock**erized basic web **s**erver (**r**ust d**ock**er **s**erver),
//! which exposes an address and port, and returns some basic response over TCP or UDP.
//!
//! it also creates a docker image when `cargo build` or `cargo run` is run.
//!
//! ## Running rocks
//! 1. Change directory to wherever rocks was installed
//! 1. In the terminal, run `cargo build` or `cargo run`
//!     - This will create a docker image for you
//! 1. rocks will announce what ip address and port it's listening on

/// [`builders`] is a collection of functionality
/// for building new [`types`], so type members can
/// remain private to themselves.
pub mod builders {
    /// [`builders::http_response`](crate::builders::http_response)
    /// contains all the [`builders`](crate::builders) which create
    /// http responses.
    pub mod http_response;
}

/// [`services`] is a collection of functionality
/// for modifying and interacting with [`types`]
/// that are created by [`builders`].
pub mod services {
    /// [`services::http_status`](crate::services::http_status)
    /// contains all the [`services`](crate::services) which
    /// interact with [`types::http_status::HttpStatus`](crate::types::http_status::HttpStatus).
    pub mod http_status;

    /// [`services::server`](crate::services::server)
    /// contains all the [`services`](crate::services) which
    /// create and leverage a TCP server.
    pub mod tcp_server;

    /// [`services::server`](crate::services::server)
    /// contains all the [`services`](crate::services) which
    /// create and leverage a UDP server.
    pub mod udp_server;
}

/// [`types`] is a collection of `struct` and `enum`
/// that is used to hold data, for [`builders`] and
/// [`services`].
pub mod types {
    /// [`types::http_status`](crate::types::http_status)
    /// contains all the [`types`](crate::types) for anything
    /// relating to an http status.
    pub mod http_status;

    pub mod http_method;

    pub mod server_endpoint;

    pub mod server;

    pub mod server_config;
}

/// [`providers`] is a collection of functions that
/// will provide data from external sources.
pub mod providers {
    /// [`providers::environment`](crate::providers::environment)
    /// contains all the values by pulling from the `config.toml` file.
    pub mod environment;

    pub mod server_config;
}
