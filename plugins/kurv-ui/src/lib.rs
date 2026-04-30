pub mod app {
    pub mod frontend;
    pub mod kurv_ui;
}

pub mod common {
    pub mod auth;
    pub mod err;
    pub mod middleware;
    pub mod extractor {
        pub mod json;
    }
}

pub mod db;

pub use app::kurv_ui::KurvUi;
