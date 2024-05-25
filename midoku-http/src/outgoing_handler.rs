use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::types::{IncomingResponse, Method};

impl From<Method> for reqwest::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Head => reqwest::Method::HEAD,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}

pub fn handle(
    method: Method,
    url: String,
    headers: Option<Vec<(String, String)>>,
    body: Option<Vec<u8>>,
) -> Result<IncomingResponse, ()> {
    let client = Client::new();

    let mut header_map = HeaderMap::new();
    if let Some(headers) = headers {
        for (header_name, header_value) in headers {
            let header_name = header_name.to_lowercase();
            let header_name = HeaderName::from_lowercase(header_name.as_bytes());
            let header_name = header_name.map_err(|_| ())?;

            let header_value = HeaderValue::from_str(&header_value);
            let header_value = header_value.map_err(|_| ())?;

            header_map.insert(header_name, header_value);
        }
    }

    let mut request_builder = client.request(method.into(), url).headers(header_map);
    if let Some(body) = body {
        request_builder = request_builder.body(body);
    }

    let response = request_builder.send().map_err(|_| ())?;

    let status_code = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap().to_string(),
            )
        })
        .collect();
    let bytes = response.bytes().map_err(|_| ())?.to_vec();

    Ok(IncomingResponse::new(status_code, headers, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_method() {
        let reqwest_method: reqwest::Method = Method::Get.into();
        assert_eq!(reqwest_method, reqwest::Method::GET);

        let reqwest_method: reqwest::Method = Method::Post.into();
        assert_eq!(reqwest_method, reqwest::Method::POST);

        let reqwest_method: reqwest::Method = Method::Put.into();
        assert_eq!(reqwest_method, reqwest::Method::PUT);

        let reqwest_method: reqwest::Method = Method::Head.into();
        assert_eq!(reqwest_method, reqwest::Method::HEAD);

        let reqwest_method: reqwest::Method = Method::Delete.into();
        assert_eq!(reqwest_method, reqwest::Method::DELETE);
    }

    const URL: &str = "https://example.com";

    #[test]
    fn test_handle() {
        let response = handle(Method::Get, URL.to_string(), None, None);
        assert!(response.is_ok());
    }

    #[test]
    fn test_handle_with_headers() {
        let headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        let response = handle(Method::Get, URL.to_string(), Some(headers), None);
        assert!(response.is_ok());
    }

    #[test]
    fn test_handle_with_body() {
        let body = vec![1, 2, 3];
        let response = handle(Method::Get, URL.to_string(), None, Some(body));
        assert!(response.is_ok());
    }

    #[test]
    fn test_handle_with_headers_and_body() {
        let headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        let body = vec![1, 2, 3];
        let response = handle(Method::Get, URL.to_string(), Some(headers), Some(body));
        assert!(response.is_ok());
    }
}
