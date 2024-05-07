#[allow(warnings)]
mod bindings;

use bindings::exports::midoku::bindings::api::{Chapter, Filter, Guest, Manga, Page};
use bindings::midoku::http::outgoing_handler::{handle, IncomingResponse, Method};
use bindings::midoku::limiter::rate_limiter::{block, set_burst, set_period_ms};

const URL: &str = "http://example.com";

struct Component;

impl Guest for Component {
    fn initialize() -> Result<(), ()> {
        // Set the rate limiter to 3 requests per second
        set_burst(3)?;
        set_period_ms(1000)?;

        Ok(())
    }

    fn get_manga_list(_filter: Vec<Filter>, _page: u32) -> Result<(Vec<Manga>, bool), ()> {
        // Block until the rate limiter allows the request
        block();

        // Send a GET request to the API
        let url = format!("{}/manga", URL);
        let response: IncomingResponse = handle(Method::Get, &url, None, None)?;

        // Get the response bytes
        let _bytes: Vec<u8> = response.bytes();

        // Parse the response bytes into a list of manga with serde or similar
        // library. This is just a placeholder to show how to use the response.
        // The actual implementation will depend on the API.

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
