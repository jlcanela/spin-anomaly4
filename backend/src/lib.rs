use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::variables;

use serde::Serialize;
use serde_json;

use api::{OrderFailed, OrderResult};

mod service;
mod auth;
mod utils;
mod repository;

/// A simple Spin HTTP component.
#[http_component]
fn main_handler(req: Request) -> anyhow::Result<Response> {
    let _subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .without_time()
        .with_ansi(false)
        .init();

    let mut router = Router::new();
    router.get("/api/config", service::handle_config);
    router.get("/api/situation", service::handle_situation);
    router.post("/api/order", service::handle_order);
    router.post_async("/api/init", service::handle_init);
    router.post("/api/clear", service::handle_clear);
    Ok(router.handle(req))

}
