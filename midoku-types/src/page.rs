use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(record)]
pub struct Page {
    pub index: u32,
    pub url: String,
    /// The base64-encoded data of the page.
    pub base64: Vec<u8>,
}
