use midoku_http;

#[tokio::test]
async fn test_handle_response_body() {
    let response = midoku_http::outgoing_handler::handle(
        midoku_http::types::Method::Get,
        "https://jsonplaceholder.typicode.com/todos/1".to_string(),
        None,
        None,
    )
    .await;
    assert!(response.is_ok());
    let response = response.unwrap();
    let bytes = response.bytes();

    let expected = r#"{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}"#;
    let actual = std::str::from_utf8(bytes).unwrap();
    assert_eq!(actual, expected);
}
