#[allow(warnings)]
mod bindings;

use bindings::exports::midoku::example_source::api::{Chapter, Guest, Manga, Page};

struct Component;

impl Guest for Component {
    fn get_manga_list(_page: u32) -> Result<(Vec<Manga>, bool), ()> {
        Err(()) // unimplemented
    }

    fn get_manga_details(_manga_id: String) -> Result<Manga, ()> {
        Err(()) // unimplemented
    }

    fn get_chapter_list(_manga_id: String) -> Result<Vec<Chapter>, ()> {
        Err(()) // unimplemented
    }

    fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>, ()> {
        Err(()) // unimplemented
    }
}

bindings::export!(Component with_types_in bindings);
