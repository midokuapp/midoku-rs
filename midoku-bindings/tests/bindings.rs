use midoku_bindings;

const EXTENSION_PATH: &str = "tests/wasm-binaries/example_extension.wasm";

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

    assert!(get_manga_list.is_err()); // Not implemented
}

#[test]
fn test_bindings_get_manga_details() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_manga_details = bindings.get_manga_details("manga_id".to_string());

    assert!(get_manga_details.is_err()); // Not implemented
}

#[test]
fn test_bindings_get_chapter_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_chapter_list = bindings.get_chapter_list("manga_id".to_string());

    assert!(get_chapter_list.is_err()); // Not implemented
}

#[test]
fn test_bindings_get_page_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH).unwrap();

    let get_page_list = bindings.get_page_list("manga_id".to_string(), "chapter_id".to_string());

    assert!(get_page_list.is_err()); // Not implemented
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
