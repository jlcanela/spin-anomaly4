use repository::Repository;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

use serde::{Deserialize, Serialize};
use serde_json;

mod repository;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    client_id: String,
    auth0_domain: String,
}

fn config() -> Result<String, String> {
    let auth0_client_id = variables::get("auth0_client_id").map_err(|e| e.to_string())?;
    let auth0_domain = variables::get("auth0_domain").map_err(|e| e.to_string())?;

    let config = Config {
        client_id: auth0_client_id,
        auth0_domain: auth0_domain,
    };

    serde_json::to_string(&config).map_err(|e| e.to_string())
}

/// A simple Spin HTTP component.
#[http_component]
fn main_handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let _subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .without_time()
        .with_ansi(false)
        .init();

    let handlers = Handlers::new();

    match req.path() {
        "/api/config" => handlers.handle_config(req),
       // "/api/stars" => handlers.handle_stars(req),
        "/api/situation" => handlers.get_situation(req),
        "/api/order" => handlers.handle_order(req),
        "/api/init" => handlers.handle_init(req),
        "/api/clear" => handlers.handle_clear(req),
        _ => {
            println!("404 path: {:?}", req.path());
            handlers.return404(req)
        }
    }
    .map_err(|e| anyhow::Error::msg(e))
}

struct Handlers {
    pub repository: Repository,
}

impl Handlers {
    pub fn new() -> Self {
        Self { repository: Repository::new() }
    }

    fn return404(self: &Self, _req: Request) -> Result<Response, String> {
        Ok(Response::builder().status(404).build())
    }

    /// A simple Spin HTTP component.
    fn handle_config(self: &Self, req: Request) -> Result<Response, String> {
        fn res(body: String, status: u16) -> Result<Response, String> {
            Ok(Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(body)
                .build())
        }

        match config() {
            Ok(c) => {
                tracing::info!("handling request2: {:?}", req.path());
                tracing::info!("successfully fetched config");
                res(c, 200)
            }
            Err(e) => {
                tracing::error!("failed to fetch config: {}", e);
                res(e, 500)
            }
        }
    }

    fn return_json<F>(self: &Self, f: F) -> Result<Response, String> 
    where F: FnOnce() -> Result<String, String> {
        let json = f();
        match json {
            Ok(success) => {
                Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(success)
                    .build())
            }
            Err(err) => {
                tracing::error!("failed to fetch json: {}", err);
                Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body("Internal Server Error")
                    .build())
            }
        }
    }

    fn get_situation(self: &Self, _req: Request) -> Result<Response, String> {
        self.return_json(|| self.repository.situation())
    }

    fn handle_order(self: &Self, req: Request) -> Result<Response, String> {
        let order_str = std::str::from_utf8(req.body()).map_err(|e| e.to_string())?;
        let status = self.repository.order(order_str.to_string()).map(|_| "{}".to_string());
        self.return_json(||status)
    }

    fn handle_init(self: &Self, _req: Request) -> Result<Response, String> {
        tracing::info!("init game");
        self.return_json(||self.repository.init_game().map(|_| "{}".to_string()))
    }

    fn handle_clear(self: &Self, _req: Request) -> Result<Response, String> {
        tracing::info!("clear game");
        self.return_json(||self.repository.clear_game().map(|_| "{}".to_string()))
    }

}
