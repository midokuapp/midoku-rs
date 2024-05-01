use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(enum)]
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
