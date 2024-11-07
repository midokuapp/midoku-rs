use serde::{Deserialize, Serialize};
use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(Serialize, Deserialize, ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(enum)]
#[repr(u8)]
pub enum Status {
    #[component(name = "unknown")]
    Unknown,
    #[component(name = "ongoing")]
    Ongoing,
    #[component(name = "completed")]
    Completed,
    #[component(name = "hiatus")]
    Hiatus,
    #[component(name = "cancelled")]
    Cancelled,
}

#[derive(Serialize, Deserialize, ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(enum)]
#[repr(u8)]
pub enum ContentRating {
    #[component(name = "safe")]
    Safe,
    #[component(name = "suggestive")]
    Suggestive,
    #[component(name = "nsfw")]
    Nsfw,
}

#[derive(Serialize, Deserialize, ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(enum)]
#[repr(u8)]
pub enum ReadingMode {
    #[component(name = "right-to-left")]
    RightToLeft,
    #[component(name = "left-to-right")]
    LeftToRight,
    #[component(name = "vertical")]
    Vertical,
    #[component(name = "scroll")]
    Scroll,
}

#[derive(Serialize, Deserialize, ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[component(record)]
pub struct Manga {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: String,
    #[component(name = "cover-url")]
    pub cover_url: String,
    #[component(name = "author-name")]
    pub author_name: String,
    #[component(name = "artist-name")]
    pub artist_name: String,
    pub categories: Vec<String>,
    pub status: Status,
    #[component(name = "content-rating")]
    pub content_rating: ContentRating,
    #[component(name = "reading-mode")]
    pub reading_mode: ReadingMode,
}
