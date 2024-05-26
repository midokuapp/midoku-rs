use midoku_bindings;
use midoku_types::manga::{ContentRating, ReadingMode, Status};

const EXTENSION_PATH: &str = "tests/example_extension.wasm";

#[test]
fn test_bindings_from_file() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH);

    assert!(bindings.is_ok());
}

#[test]
fn test_bindings_initialize() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let initialize = bindings.initialize();

    assert!(initialize.is_ok());
}

#[test]
fn test_bindings_get_manga_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_manga_list = bindings.get_manga_list(vec![], 0);

    assert!(get_manga_list.is_ok());

    let (manga_list, has_next) = get_manga_list.unwrap();

    // Example result
    assert!(manga_list.is_empty());
    assert!(!has_next);
}

#[test]
fn test_bindings_get_manga_details() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_manga_details = bindings.get_manga_details("manga_id".to_string());

    assert!(get_manga_details.is_ok());

    let manga = get_manga_details.unwrap();

    // Example result
    assert_eq!(manga.id, "");
    assert_eq!(manga.title, "");
    assert_eq!(manga.url, "");
    assert_eq!(manga.description, "");
    assert_eq!(manga.cover_url, "");
    assert_eq!(manga.author_name, "");
    assert_eq!(manga.artist_name, "");
    assert!(manga.categories.is_empty());
    assert_eq!(manga.status, Status::Unknown);
    assert_eq!(manga.content_rating, ContentRating::Safe);
    assert_eq!(manga.reading_mode, ReadingMode::RightToLeft);
}

#[test]
fn test_bindings_get_chapter_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_chapter_list = bindings.get_chapter_list("manga_id".to_string());

    assert!(get_chapter_list.is_ok());

    let chapter_list = get_chapter_list.unwrap();

    // Example result
    assert!(chapter_list.is_empty());
}

#[test]
fn test_bindings_get_page_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_page_list = bindings.get_page_list("manga_id".to_string(), "chapter_id".to_string());

    assert!(get_page_list.is_ok());

    let page_list = get_page_list.unwrap();

    // Example result
    assert!(page_list.is_empty());
}

#[test]
fn test_bindings_settings() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let settings = bindings.settings();

    assert!(settings.is_empty());
}

#[test]
fn test_bindings_setting_mut() {
    let mut bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    bindings.settings_mut().insert(
        "key1".to_string(),
        midoku_settings::types::Value::String("value1".to_string()),
    );
    bindings.settings_mut().insert(
        "key2".to_string(),
        midoku_settings::types::Value::String("value2".to_string()),
    );

    let settings = bindings.settings();

    let value1 = settings.get("key1");
    let value2 = settings.get("key2");
    let value3 = settings.get("key3");

    assert_eq!(
        value1,
        Some(&midoku_settings::types::Value::String("value1".to_string()))
    );
    assert_eq!(
        value2,
        Some(&midoku_settings::types::Value::String("value2".to_string()))
    );
    assert_eq!(value3, None);
}
