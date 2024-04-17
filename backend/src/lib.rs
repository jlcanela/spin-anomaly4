use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

use serde::{Deserialize, Serialize};
use serde_json;

use api::Star;

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
fn handle_my_rust_app(req: Request) -> anyhow::Result<impl IntoResponse> {
    match req.path() {
        "/api/config" => handle_config(req),
        "/api/stars" => handle_stars(req),
        "/api/order" => handle_order(req),
        "/api/init" => handle_init(req),
        _ => {
            println!("404 path: {:?}", req.path());
            return404(req)
        }
    }.map_err(|e| anyhow::Error::msg(e))
}

fn return404(_req: Request) -> Result<Response, String> {
    Ok(Response::builder()
    .status(404)
    .build())
}

/// A simple Spin HTTP component.
fn handle_config(req: Request) -> Result<Response, String> {

    let _subscriber = tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .without_time()
    .init();

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
        },
        Err(e) => {
            tracing::error!("failed to fetch config: {}", e);
            res(e, 500)
        }
    }

}

fn stars() -> Vec<Star> {
    vec!(
        Star { id: 0, name: "Rataxi".to_string(), owner: "Yellow".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 1, name: "Rogafo".to_string(), owner: "Green".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 2, name: "Rikidi".to_string(), owner: "Blue".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 3, name: "Naove".to_string(), owner: "Red".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 4, name: "Gimani".to_string(), owner: "Purple".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 5, name: "Tatufo".to_string(), owner: "Orange".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
    )
}

/// A simple Spin HTTP component.
fn handle_stars(_req: Request) -> Result<Response, String> {

    let st = stars();
    let json = serde_json::to_string(&st).map_err(|e| e.to_string())?;

    Ok(Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(json)
    .build())
}

/// A simple Spin HTTP component.
fn handle_order(_req: Request) -> Result<Response, String> {
    Ok(Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body("{}")
    .build())
}

/// A simple Spin HTTP component.
fn handle_init(_req: Request) -> Result<Response, String> {
    Ok(Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body("{ \"result\": \"ok\" }")
    .build())
}
