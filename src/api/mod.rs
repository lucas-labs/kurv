pub mod eggs;
pub mod err;
pub mod status;

use {
    crate::common::tcp::{err, handle as handle_tcp, Handler, Request, Response},
    crate::kurv::{InfoMtx, KurvStateMtx},
    anyhow::Result,
    log::info,
    std::net::TcpListener,
};

pub struct Context {
    pub state: KurvStateMtx,
    pub info: InfoMtx,
}

type RouteHandler = fn(&Request, &Context) -> Result<Response>;
type RouteRegex = &'static str;
type RouteMethod = &'static str;
type RouteDef = (RouteMethod, RouteRegex, RouteHandler);

struct Router {
    info: InfoMtx,
    state: KurvStateMtx,
}

impl Router {
    /// returns a list of routes which are composed of a method and a regex path
    fn routes(&self) -> Vec<RouteDef> {
        vec![
            ("GET", "/", status::status),
            ("GET", "/status", status::status),
            ("GET", "/eggs", eggs::summary),
            ("POST", "/eggs", eggs::collect),
            ("POST", "/eggs/(?P<egg_id>.*)/stop", eggs::stop),
            ("POST", "/eggs/(?P<egg_id>.*)/start", eggs::start),
            ("POST", "/eggs/(?P<egg_id>.*)/restart", eggs::restart),
            ("POST", "/eggs/(?P<egg_id>.*)/remove", eggs::remove),
            ("POST", "/eggs/(?P<egg_id>.*)/egg", eggs::remove),
            ("GET", "/eggs/(?P<egg_id>.*)", eggs::get),
            (".*", ".*", err::not_allowed), // last resort
        ]
    }

    fn compiled_routes(&self) -> Vec<(regex_lite::Regex, RouteHandler)> {
        self.routes()
            .iter()
            .map(|&(method, regex_raw, handler)| {
                let route_re = regex_lite::Regex::new(format!("^{method} {regex_raw}/?$").as_str())
                    .expect("Invalid regex pattern on route");
                (route_re, handler)
            })
            .collect()
    }
}

impl Handler for Router {
    fn handle(&self, request: &mut Request) -> Response {
        let method = request.method.as_str();
        let path = request.path.as_str();
        // let mut request = request.clone();
        let compiled_routes = self.compiled_routes();

        let mut result = err(500, "internal server error".to_string());

        for (route_re, handler) in compiled_routes {
            let route = format!("{method} {path}");
            let route_str = route.as_str();
            let names = route_re.capture_names().flatten();

            if let Some(capture) = route_re.captures(route_str) {
                for key in names {
                    let value = capture.name(key).map(|v| v.as_str()).unwrap_or("");
                    request.path_params.insert(key.to_string(), value.to_string());
                }

                let ctx = Context {
                    state: self.state.clone(),
                    info: self.info.clone(),
                };
                result = match handler(request, &ctx) {
                    Ok(response) => response,
                    Err(e) => err(500, format!("{}", e)),
                };
                break;
            }
        }

        result
    }
}

/// starts the api server
pub fn start(info: InfoMtx, state: KurvStateMtx) {
    let host = std::env::var("KURV_API_HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("KURV_API_PORT").unwrap_or("58787".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    info!(
        "<head>kurv</head> api listening on <green>http://{}:{}/</green>",
        host, port
    );

    let router = Router { info, state };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_tcp(stream, &router);
    }
}
