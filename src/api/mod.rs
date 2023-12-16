mod eggs;
mod err;
mod status;

use {
    crate::common::tcp::{err, handle as handle_tcp, Handler, Request, Response},
    crate::kurv::{InfoMtx, KurvStateMtx},
    log::info,
    std::net::TcpListener,
};

struct RouterHandler {
    info: InfoMtx,
    state: KurvStateMtx,
}

impl Handler for RouterHandler {
    fn handle(&self, request: &Request) -> Response {
        let method = request.method.as_str();
        let path = request.path.as_str();

        let result = match (method, path) {
            ("GET", "/") | ("GET", "/status") => self.status(request),
            ("GET", "/eggs") => self.list_eggs(request),
            _ => self.not_allowed(request),
        };

        match result {
            Ok(response) => response,
            Err(e) => err(500, format!("{}", e)),
        }
    }
}

/// starts the api server
pub fn start(info: InfoMtx, state: KurvStateMtx) {
    let host = std::env::var("KURV_API_HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("KURV_API_PORT").unwrap_or("5878".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    info!(
        "<head>kurv</head> api listening on <green>http://{}:{}/</green>",
        host, port
    );

    let router = RouterHandler { info, state };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_tcp(stream, &router);
    }
}
