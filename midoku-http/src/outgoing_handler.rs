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
