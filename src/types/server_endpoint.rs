use super::http_method::HttpMethod;
use toml::Value;

/// [`ServerEndpoint`] is a [`struct`] that is populated
/// with a path and an [`HttpMethod`].
#[derive(Debug, serde::Deserialize)]
pub struct ServerEndpoint {
    pub path: String,
    pub method: HttpMethod,
    pub response: String,
}

impl ServerEndpoint {
    pub fn from_toml(value: &Value) -> Self {
        println!("{:?}", value);
        let path = value
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| panic!("Missing or invalid `path` field."));
        let method = value
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| panic!("Missing or invalid `method` field."));
        let response = value
            .get("response")
            .and_then(|v| v.as_str())
            .ok_or_else(|| panic!("Missing or invalid `response` field."));

        ServerEndpoint {
            path: path.unwrap().to_string(),
            method: HttpMethod::coerce(method.unwrap()),
            response: response.unwrap().to_string(),
        }
    }
}
