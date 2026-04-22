use {
    axum::http::{HeaderValue, Method, header::SERVER},
    tower::{
        ServiceBuilder,
        layer::util::{Identity, Stack},
    },
    tower_http::{cors::CorsLayer, set_header::SetResponseHeaderLayer},
};

/// Creates a standard middleware stack with CORS and server header
pub fn stack()
-> ServiceBuilder<Stack<SetResponseHeaderLayer<HeaderValue>, Stack<CorsLayer, Identity>>> {
    let server_header_value = HeaderValue::from_static("kurv-ui-server");

    ServiceBuilder::new()
        .layer(CorsLayer::new().allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .layer(SetResponseHeaderLayer::if_not_present(SERVER, server_header_value))
}
