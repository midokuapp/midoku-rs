use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(enum)]
#[repr(u8)]
pub enum Method {
    #[component(name = "get")]
    Get,
    #[component(name = "post")]
    Post,
    #[component(name = "put")]
    Put,
    #[component(name = "head")]
    Head,
    #[component(name = "delete")]
    Delete,
}

#[derive(Debug)]
pub struct IncomingResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    bytes: Vec<u8>,
}

impl IncomingResponse {
    pub(crate) fn new(status_code: u16, headers: Vec<(String, String)>, bytes: Vec<u8>) -> Self {
        IncomingResponse {
            status_code,
            headers,
            bytes,
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incoming_response() {
        let status_code = 200;
        let headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        let bytes = vec![1, 2, 3];

        let response = IncomingResponse::new(status_code, headers.clone(), bytes.clone());

        assert_eq!(response.status_code(), status_code);
        assert_eq!(response.headers(), &headers);
        assert_eq!(response.bytes(), &bytes);
    }
}
