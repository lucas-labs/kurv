mod eggs;
mod err;
mod status;

use std::net::TcpListener;

use crate::{
    common::{
        self,
        tcp::{err, Handler, Request, Response},
    },
    kurv::{InfoMtx, KurvStateMtx},
    printth,
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

// pub fn router(request: &Request) -> Handler {
//     let method = request.method.as_str();
//     let path = request.path.as_str();

//     match (method, path) {
//         ("GET", "/") | ("GET", "/ping") => status::handle,
//         ("GET", "/hello") => hello::handle,
//         _ => err::not_allowed,
//     }
// }

/// starts the api server
pub fn start(info: InfoMtx, state: KurvStateMtx) {
    let host = std::env::var("KURV_API_HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("KURV_API_PORT").unwrap_or("3247".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    printth!(
        "<highlight>$</highlight> <head>kurv</head> api listening on <green>http://{}:{}/</green>",
        host,
        port
    );

    let router = RouterHandler { info, state };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        common::tcp::handle(stream, &router);
    }
}
