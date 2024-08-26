use crate::services::http_status::HttpStatusService;
use crate::types::http_status::HttpStatus;

/// [`create_http_response`] creates an http header that contains
/// a `return_code` created from an [`HttpStatus`]), and the
/// provided `content`, then returns the result as a [`String`].
///
/// # example
/// ```rust
/// use rocks::types::http_status::HttpStatus;
/// use rocks::builders::http_response::create_http_response;
///
/// fn get_http_response(return_code: HttpStatus, content: &str) -> String {
///     create_http_response(return_code, content)
/// }
/// ```
pub fn create_http_response(return_code: HttpStatus, content: &str) -> String {
    let http_status = return_code.get_status();
    format!("HTTP/1.1 {http_status}\r\nContent-Length: {}\r\nCache: no-cache\r\nVary: Cookie, Accept-Encoding\r\nContent-Type: text/html\r\n\r\n{content}", content.len())
}
