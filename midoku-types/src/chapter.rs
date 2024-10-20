use serde::{Deserialize, Serialize};
use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(Serialize, Deserialize, ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[component(record)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub volume: f32,
    pub chapter: f32,

    /// The date the chapter was last updated. This is a Unix timestamp in seconds.
    #[component(name = "date-updated")]
    pub date_updated: u32,
    pub scanlator: String,
    pub url: String,
    pub language: String,
}
