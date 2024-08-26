/// [`HttpStatus`] is a list of pre-defined http statuses
/// that the Rust application will handle.
pub enum HttpStatus {
    /// An "OK" [`HttpStatus`], used to generate an
    /// `HTTP/1.1 200 OK` header.
    Ok,
    /// A "Page Not Found" [`HttpStatus`] that signals
    /// a URL has been hit that does not exist, used to
    /// generate an `HTTP/1.1 400 Page Not Found` header.
    NotFound,
}
