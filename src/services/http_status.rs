use crate::types::http_status::HttpStatus;

/// [`HttpStatusService`] is a collection of functionality that
/// interacts with an [`HttpStatus`].
pub trait HttpStatusService {
    fn get_status(&self) -> String;
}

/// Implement [`HttpStatusService`] for an [`HttpStatus`].
impl HttpStatusService for HttpStatus {
    /// [`get_status`](HttpStatusService::get_status)  will return a
    /// [`String`] representation of an [`HttpStatus`].
    ///
    /// # example
    /// ```rust
    /// use rocks::services::http_status::HttpStatusService;
    /// use rocks::types::http_status::HttpStatus;
    ///
    /// fn get_http_status_header(status: HttpStatus) -> String {
    ///     status.get_http_status()
    /// }
    /// ```
    fn get_status(&self) -> String {
        match self {
            HttpStatus::Ok => "200 OK",
            HttpStatus::NotFound => "404 NOT FOUND",
        }
        .to_string()
    }
}
