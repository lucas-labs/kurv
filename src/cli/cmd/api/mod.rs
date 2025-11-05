mod eggs;

use {
    crate::common::tcp::ErrorResponse,
    anyhow::{Result, anyhow},
    serde::Deserialize,
    std::{
        io::{Read, Write},
        net::TcpStream,
        str,
    },
};

// ApiResponse struct to hold response headers and body
pub(crate) struct ApiResponse {
    #[allow(dead_code)]
    pub headers: String,
    pub body: String,
}

// Api struct with host and port fields
pub struct Api {
    pub host: String,
    pub port: u16,
}

impl Api {
    pub fn new() -> Self {
        let host = std::env::var("KURV_API_HOST").unwrap_or("127.0.0.1".to_string());
        let port = std::env::var("KURV_API_PORT")
            .unwrap_or("58787".to_string())
            .parse::<u16>()
            .unwrap_or(5878);

        Api { host, port }
    }

    // Private helper method to perform HTTP request and get response
    fn request(&self, method: &str, path: &str, body: Option<&str>) -> Result<ApiResponse> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))
            .map_err(|_| anyhow!("failed to connect to api server"))?;

        let body_str = match body {
            Some(b) => format!("Content-Length: {}\r\n\r\n{}", b.len(), b),
            None => String::from("\r\n"),
        };

        let request =
            format!("{} {} HTTP/1.1\r\nHost: {}\r\n{}\r\n", method, path, self.host, body_str);

        stream
            .write_all(request.as_bytes())
            .map_err(|_| anyhow!("failed to write to api server"))?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).map_err(|_| anyhow!("failed to read from api server"))?;

        let response_str = str::from_utf8(&buffer)
            .map_err(|_| anyhow!("failed to parse response from api server"))?;

        // Extract headers and body from the response string
        let mut header_body_split = response_str.split("\r\n\r\n");
        let headers = header_body_split.next().unwrap_or_default().to_string();
        let body = header_body_split.next().unwrap_or_default().to_string();

        Ok(ApiResponse { headers, body })
    }

    // Method to perform HTTP GET request
    pub(crate) fn get(&self, path: &str) -> Result<ApiResponse> {
        self.request("GET", path, None)
    }

    // Method to perform HTTP POST request
    pub(crate) fn post(&self, path: &str, body: &str) -> Result<ApiResponse> {
        self.request("POST", path, Some(body))
    }

    // Method to perform HTTP PUT request
    #[allow(dead_code)]
    pub(crate) fn put(&self, path: &str, body: &str) -> Result<ApiResponse> {
        self.request("PUT", path, Some(body))
    }

    // Method to perform HTTP DELETE request
    #[allow(dead_code)]
    pub(crate) fn delete(&self, path: &str) -> Result<ApiResponse> {
        self.request("DELETE", path, None)
    }
}

pub enum ParsedResponse<T> {
    Success(T),
    Failure(ErrorResponse),
}

/// parses a response from the server api.
///
/// It returns a `ParsedResponse` that can either be a success call of type `T`
/// or a failure of type `ErrorResponse`
pub fn parse_response<'a, T: Deserialize<'a>>(
    response: &'a ApiResponse,
) -> Result<ParsedResponse<T>> {
    let maybe_egg: Result<T, _> = serde_json::from_str(response.body.as_str());

    match maybe_egg {
        Ok(parsed) => Ok(ParsedResponse::Success(parsed)),
        Err(_) => {
            // try to parse it as an ErrorResponse.
            let maybe_err_resp: Result<ErrorResponse, _> =
                serde_json::from_str(response.body.as_str());

            match maybe_err_resp {
                Ok(parsed) => Ok(ParsedResponse::Failure(parsed)),
                Err(_) => Err(anyhow!("couldn't parse kurv server response")),
            }
        }
    }
}
