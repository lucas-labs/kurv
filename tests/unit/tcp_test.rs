use {
    kurv::common::tcp::{Request, Response, json},
    serde_json::json,
    std::collections::HashMap,
};

#[test]
fn test_request_creation() {
    let request = Request {
        method: "GET".to_string(),
        path: "/eggs".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec!["Content-Type: application/json".to_string()],
        body: "".to_string(),
        query_params: HashMap::new(),
        path_params: HashMap::new(),
    };

    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/eggs");
    assert_eq!(request.version, "HTTP/1.1");
}

#[test]
fn test_query_params() {
    let mut query_params = HashMap::new();
    query_params.insert("id".to_string(), "1".to_string());
    query_params.insert("status".to_string(), "running".to_string());

    let request = Request {
        method: "GET".to_string(),
        path: "/eggs".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![],
        body: "".to_string(),
        query_params: query_params.clone(),
        path_params: HashMap::new(),
    };

    assert_eq!(request.query_params.get("id"), Some(&"1".to_string()));
    assert_eq!(request.query_params.get("status"), Some(&"running".to_string()));
}

#[test]
fn test_path_params() {
    let mut path_params = HashMap::new();
    path_params.insert("egg_id".to_string(), "test-egg".to_string());

    let request = Request {
        method: "POST".to_string(),
        path: "/eggs/test-egg/stop".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec![],
        body: "".to_string(),
        query_params: HashMap::new(),
        path_params: path_params.clone(),
    };

    assert_eq!(request.path_params.get("egg_id"), Some(&"test-egg".to_string()));
}

#[test]
fn test_json_response_helper() {
    #[derive(serde::Serialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    let data = TestData {
        name: "test".to_string(),
        value: 42,
    };

    let response = json(200, data);

    assert_eq!(response.status, 200);
    assert!(response.headers.contains(&"Content-Type: application/json".to_string()));

    // verify body is valid JSON
    let body_str = String::from_utf8(response.body).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(parsed["name"], "test");
    assert_eq!(parsed["value"], 42);
}

#[test]
fn test_response_creation() {
    let response = Response {
        status: 200,
        headers: vec!["Content-Type: text/plain".to_string()],
        body: b"Hello, World!".to_vec(),
    };

    assert_eq!(response.status, 200);
    assert_eq!(response.headers.len(), 1);
    assert_eq!(String::from_utf8(response.body).unwrap(), "Hello, World!");
}

#[test]
fn test_response_with_json_body() {
    let json_body = json!({"status": "ok", "count": 5});
    let body_bytes = serde_json::to_vec(&json_body).unwrap();

    let response = Response {
        status: 200,
        headers: vec!["Content-Type: application/json".to_string()],
        body: body_bytes,
    };

    assert_eq!(response.status, 200);

    // verify we can deserialize the body
    let parsed: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
    assert_eq!(parsed["status"], "ok");
    assert_eq!(parsed["count"], 5);
}

#[test]
fn test_multiple_headers() {
    let response = Response {
        status: 200,
        headers: vec![
            "Content-Type: application/json".to_string(),
            "X-Custom-Header: value".to_string(),
            "Cache-Control: no-cache".to_string(),
        ],
        body: vec![],
    };

    assert_eq!(response.headers.len(), 3);
    assert!(response.headers.contains(&"Content-Type: application/json".to_string()));
    assert!(response.headers.contains(&"X-Custom-Header: value".to_string()));
    assert!(response.headers.contains(&"Cache-Control: no-cache".to_string()));
}

#[test]
fn test_request_with_body() {
    let body = r#"{"name": "new-egg", "command": "echo"}"#;

    let request = Request {
        method: "POST".to_string(),
        path: "/eggs".to_string(),
        version: "HTTP/1.1".to_string(),
        headers: vec!["Content-Type: application/json".to_string()],
        body: body.to_string(),
        query_params: HashMap::new(),
        path_params: HashMap::new(),
    };

    assert_eq!(request.method, "POST");
    assert!(!request.body.is_empty());

    // verify body is valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&request.body).unwrap();
    assert_eq!(parsed["name"], "new-egg");
    assert_eq!(parsed["command"], "echo");
}
