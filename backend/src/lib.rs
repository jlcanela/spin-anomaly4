
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

use serde::{Deserialize, Serialize};
use serde_json;

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
fn handle_my_rust_app(_req: Request) -> anyhow::Result<impl IntoResponse> {

    fn res(body: String, status: u16) -> anyhow::Result<impl IntoResponse> {
      Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
    }

    match config() {
        Ok(c) => res(c, 200),
        Err(e) => res(e, 500),
    }

}
