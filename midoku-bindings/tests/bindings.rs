use std::path::PathBuf;
use std::sync::LazyLock;

use midoku_bindings;
use midoku_types::manga::{ContentRating, ReadingMode, Status};

static EXTENSION_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    // Build the extension in release mode
    let output = std::process::Command::new("cargo-component")
        .args(&[
            "build",
            "--release",
            "--package",
            "example-extension",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("Failed to build the extension");

    if !output.status.success() {
        panic!(
            "Failed to build the extension: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Get the built extension path
    let output = std::process::Command::new(env!("CARGO"))
        .args(&["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .expect("Failed to find workspace root")
        .stdout;
    let raw_path = std::str::from_utf8(&output).unwrap().trim();
    let workspace_path = PathBuf::from(raw_path).parent().unwrap().to_path_buf();

    workspace_path.join("target/wasm32-unknown-unknown/release/example_extension.wasm")
});

#[tokio::test]
async fn test_bindings_from_file() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path()).await;

    assert!(bindings.is_ok());
}

#[tokio::test]
async fn test_bindings_initialize() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let initialize = bindings.initialize().await;

    assert!(initialize.is_ok());
}

#[tokio::test]
async fn test_bindings_get_manga_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let get_manga_list = bindings.get_manga_list(vec![], 0).await;

    assert!(get_manga_list.is_ok());

    let (manga_list, has_next) = get_manga_list.unwrap();

    // Example result
    assert!(manga_list.is_empty());
    assert!(!has_next);
}

#[tokio::test]
async fn test_bindings_get_manga_details() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let get_manga_details = bindings.get_manga_details("manga_id".to_string()).await;

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

#[tokio::test]
async fn test_bindings_get_chapter_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let get_chapter_list = bindings.get_chapter_list("manga_id".to_string()).await;

    assert!(get_chapter_list.is_ok());

    let chapter_list = get_chapter_list.unwrap();

    // Example result
    assert!(chapter_list.is_empty());
}

#[tokio::test]
async fn test_bindings_get_page_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let get_page_list = bindings
        .get_page_list("manga_id".to_string(), "chapter_id".to_string())
        .await;

    assert!(get_page_list.is_ok());

    let page_list = get_page_list.unwrap();

    // Example result
    assert!(page_list.is_empty());
}

#[tokio::test]
async fn test_bindings_settings() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    let settings = bindings.settings().await;

    assert!(settings.is_empty());
}

#[tokio::test]
async fn test_bindings_setting_mut() {
    let mut bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path())
        .await
        .unwrap();

    bindings.settings_mut().await.insert(
        "key1".to_string(),
        midoku_settings::types::Value::String("value1".to_string()),
    );
    bindings.settings_mut().await.insert(
        "key2".to_string(),
        midoku_settings::types::Value::String("value2".to_string()),
    );

    let settings = bindings.settings().await;

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
