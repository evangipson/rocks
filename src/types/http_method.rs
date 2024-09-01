/// [`HttpMethod`] is a list of pre-defined http methods
/// that [`rocks`] will support.
#[derive(PartialEq, Debug, serde::Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    pub fn coerce(method: &str) -> Self {
        match method.to_lowercase().as_str() {
            "get" => HttpMethod::GET,
            "post" => HttpMethod::POST,
            // Add other methods as needed
            _ => panic!("Could not parse http method."),
        }
    }
}
