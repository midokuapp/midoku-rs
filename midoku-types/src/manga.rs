use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(enum)]
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

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(enum)]
pub enum ContentRating {
    #[component(name = "safe")]
    Safe,
    #[component(name = "suggestive")]
    Suggestive,
    #[component(name = "nsfw")]
    Nsfw,
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(enum)]
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

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
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
