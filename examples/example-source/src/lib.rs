#[allow(warnings)]
mod bindings;

use bindings::{Chapter, Guest, Manga, Page, PageResult};

struct Component;

impl Guest for Component {
    fn get_manga_list(_page: u32) -> Result<PageResult, ()> {
        unimplemented!()
    }

    fn get_manga_details(_manga_id: String) -> Result<Manga, ()> {
        unimplemented!()
    }

    fn get_chapter_list(_manga_id: String) -> Result<Vec<Chapter>, ()> {
        unimplemented!()
    }

    fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>, ()> {
        unimplemented!()
    }
}

bindings::export!(Component with_types_in bindings);
