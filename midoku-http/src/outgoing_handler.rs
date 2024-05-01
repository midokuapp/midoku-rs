use crate::types::{IncomingResponse, Method};

pub fn handle(
    method: Method,
    url: String,
    headers: Option<Vec<(String, String)>>,
    body: Option<Vec<u8>>,
) -> Result<IncomingResponse, ()> {
    todo!()
}
