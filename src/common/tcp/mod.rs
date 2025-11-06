use {
    log::trace,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashMap,
        fmt::Display,
        io::{BufReader, Read, Write, prelude::BufRead},
        net::TcpStream,
    },
};

/// List of common HTTP methods mapped to their string representations.
const RESPONSE_CODES: [(u16, &str); 14] = [
    (200, "OK"),
    (201, "Created"),
    (202, "Accepted"),
    (204, "No Content"),
    (400, "Bad Request"),
    (401, "Unauthorized"),
    (403, "Forbidden"),
    (404, "Not Found"),
    (405, "Method Not Allowed"),
    (409, "Conflict"),
    (418, "I'm a teapot"),
    (500, "Internal Server Error"),
    (501, "Not Implemented"),
    (505, "HTTP Version Not Supported"),
];

/// Returns the string representation of the given status code.
fn get_status_text(status: u16) -> String {
    RESPONSE_CODES
        .iter()
        .find(|&&(code, _)| code == status)
        .map(|&(_, text)| text)
        .unwrap_or("Unknown Status")
        .to_string()
}

/// A Request is a struct that holds the request data
/// and is passed to the handler function.
#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<String>,
    pub body: String,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}

impl Display for Request {
    /// format as yaml
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let yaml = serde_saphyr::to_string(&self).unwrap();
        write!(f, "{}", yaml)
    }
}

/// A Response is a struct that holds the response data
/// and is returned from the handler function.
pub struct Response {
    pub status: u16,
    pub headers: Vec<String>,
    pub body: Vec<u8>,
}

/// common error response
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
}

pub trait Handler {
    fn handle(&self, request: &mut Request) -> Response;
}

/// Returns a JSON response with the given body and status code.
pub fn json<T: Serialize>(status: u16, body: T) -> Response {
    let body = serde_json::to_vec(&body).unwrap();

    Response {
        status,
        headers: vec!["Content-Type: application/json".to_string()],
        body,
    }
}

pub fn err(status: u16, msg: String) -> Response {
    json(
        status,
        ErrorResponse {
            code: status,
            status: get_status_text(status),
            message: msg,
        },
    )
}

/// Handles an incoming TCP connection stream.
#[allow(clippy::sliced_string_as_bytes)]
pub fn handle(mut stream: TcpStream, handler: &impl Handler) {
    let mut buf_reader = BufReader::new(&mut stream);

    // Read the request line
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts[0].to_string();
    let full_path = parts[1].to_string();

    let trim: &[_] = &['\r', '\n'];
    trace!("{}", request_line.trim_matches(trim));

    // Extract path and query parameters
    let (path, query_params) = match full_path.find('?') {
        Some(index) => {
            let (path, query_string) = full_path.split_at(index);
            let query_params: HashMap<String, String> =
                form_urlencoded::parse(query_string[1..].as_bytes())
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect();
            (path.to_string(), query_params)
        }
        None => (full_path, HashMap::new()),
    };

    // Read the headers
    let mut headers = Vec::new();
    loop {
        let mut header_line = String::new();
        buf_reader.read_line(&mut header_line).unwrap();
        if header_line.trim().is_empty() {
            break;
        }
        headers.push(header_line.trim().to_string());
    }

    // Check for Content-Length header
    let content_length: usize = headers
        .iter()
        .find_map(|header| {
            if header.to_lowercase().starts_with("content-length:") {
                header.split_whitespace().last().and_then(|len| len.parse().ok())
            } else {
                None
            }
        })
        .unwrap_or(0);

    // Read the body if Content-Length is present
    let mut body = String::new();
    if content_length > 0 {
        let mut body_bytes = vec![0u8; content_length];
        buf_reader.read_exact(&mut body_bytes).unwrap();
        body = String::from_utf8_lossy(&body_bytes).to_string();
    }

    let mut request = Request {
        headers,
        method,
        path,
        version: "HTTP/1.1".to_string(),
        body,
        query_params,
        path_params: HashMap::new(),
    };

    // Handle the request and get the response
    let response = handler.handle(&mut request);

    let http_response = format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        response.status,
        get_status_text(response.status),
        get_headers(response.headers, &response.body),
        String::from_utf8_lossy(&response.body)
    );

    // Write the HTTP response to the stream
    stream.write_all(http_response.as_bytes()).unwrap();
}

/// Returns the final headers string including content-length and other defaults.
fn get_headers(user_headers: Vec<String>, body: &[u8]) -> String {
    let mut headers = Vec::new();
    headers.push("Server: kurv".to_string());
    headers.push(format!("Content-Length: {}", body.len()));
    headers.push(format!("Date: {}", chrono::Utc::now().to_rfc2822()));

    // cors headers
    headers.push("Access-Control-Allow-Origin: *".to_string());
    headers.push("Access-Control-Allow-Methods: GET, POST, OPTIONS".to_string());
    headers.push("Access-Control-Allow-Headers: Content-Type".to_string());

    headers.extend(user_headers);
    headers.join("\r\n")
}
