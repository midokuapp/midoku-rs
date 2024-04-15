#[allow(warnings)]
mod bindings;

mod client;
mod response;

use reqwest::Method as ReqwestMethod;

use crate::bindings::exports::midoku::midoku_http::http;

use crate::client::Client;
use crate::response::Response;

impl From<http::Method> for ReqwestMethod {
    fn from(value: http::Method) -> Self {
        match value {
            http::Method::Get => ReqwestMethod::GET,
            http::Method::Post => ReqwestMethod::POST,
            http::Method::Put => ReqwestMethod::PUT,
            http::Method::Head => ReqwestMethod::HEAD,
            http::Method::Delete => ReqwestMethod::DELETE,
        }
    }
}

struct Component;

impl http::Guest for Component {
    type Client = Client;
    type Response = Response;
}

bindings::export!(Component with_types_in bindings);
