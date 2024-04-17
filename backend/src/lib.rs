use std::time::SystemTime;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;
use spin_sdk::key_value::Store;

use serde::{Deserialize, Serialize};
use serde_json;

use api::{Player, Situation, Star};

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
    let _subscriber = tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .without_time()
    .with_ansi(false)
    .init();

    match req.path() {
        "/api/config" => handle_config(req),
        "/api/stars" => handle_stars(req),
        "/api/situation" => handle_situation(req),
        "/api/order" => handle_order(req),
        "/api/init" => handle_init(req),
        "/api/clear" => handle_clear(req),
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
        Star { id: 0, name: "Rataxi".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 1, name: "Rogafo".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 2, name: "Rikidi".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 3, name: "Naove".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 4, name: "Gimani".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 5, name: "Tatufo".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
    )
}

fn radar() -> Vec<Star> {
    vec!(
        Star { id: 6, name: "Godami".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 7, name: "Lulubi".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 8, name: "Daraki".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
    )
}

fn players() -> Vec<Player> {
    vec!(
        Player { id: 0, sub: None, name: "Empire".to_string(), points: 0, last_update: SystemTime::now() },
        Player { id: 1, sub: Some("google-oauth2|112177133653097483983".to_string()), name: "Player".to_string(), points: 0, last_update: SystemTime::now() },
    )
}

fn load_from_kv<T: for<'de> serde::Deserialize<'de>>(name: String, store: &Store) -> Result<T, String> 
    {
    let binary = store.get(&name).map_err(|e| e.to_string())?;
    let buffer = binary.ok_or(format!("no game ({})", name))?;
    let str = String::from_utf8(buffer).map_err(|e| e.to_string())?;
    let res = serde_json::from_str::<T>(&str).map_err(|e| e.to_string())?;
    Ok(res)
}
/// A simple Spin HTTP component.
fn handle_situation(_req: Request) -> Result<Response, String> {

    let store = Store::open_default().map_err(|e| e.to_string())?;
    let stars = load_from_kv::<Vec<Star>>("stars".to_string(), &store)?;
    let radar = load_from_kv::<Vec<Star>>("radar".to_string(), &store)?;
    let players = load_from_kv::<Vec<Player>>("players".to_string(), &store)?;

    let situation = Situation {
        player: players[1].clone(),
        stars: stars,
        radar: radar,
    };

    let json = serde_json::to_string(&situation).map_err(|e| e.to_string())?;

    Ok(Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(json)
    .build())
}

/// A simple Spin HTTP component.
fn handle_stars(_req: Request) -> Result<Response, String> {

    let store = Store::open_default().map_err(|e| e.to_string())?;
    let value = store.get("stars").map_err(|e| e.to_string())?;
    let buffer = value.ok_or("no game")?;
    let json = String::from_utf8(buffer).map_err(|e| e.to_string())?;
   
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

fn init_game() -> Result<(), String> {
    let store = Store::open_default().map_err(|e| e.to_string())?;

    let stars = serde_json::to_string(&stars()).map_err(|e| e.to_string())?;
    store.set("stars", stars.as_bytes()).map_err(|e| e.to_string())?;

    let radar = serde_json::to_string(&radar()).map_err(|e| e.to_string())?;
    store.set("radar", radar.as_bytes()).map_err(|e| e.to_string())?;

    let players = serde_json::to_string(&players()).map_err(|e| e.to_string())?;
    store.set("players", players.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

fn clear_game() -> Result<(), String> {
    let store = Store::open_default().map_err(|e| e.to_string())?;
    store.delete("stars").map_err(|e| e.to_string())
}

/// A simple Spin HTTP component.
fn handle_init(_req: Request) -> Result<Response, String> {
    tracing::info!("init game");

    if init_game().is_ok() {
        Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{ \"result\": \"ok\" }")
        .build())
    } else {
        Ok(Response::builder()
        .status(500)
        .header("content-type", "application/json")
        .body("{ \"result\": \"error\" }")
        .build())
    }    

}

/// A simple Spin HTTP component.
fn handle_clear(_req: Request) -> Result<Response, String> {
    tracing::info!("clear game");

    if clear_game().is_ok() {
        Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{ \"result\": \"ok\" }")
        .build())
    } else {
        Ok(Response::builder()
        .status(500)
        .header("content-type", "application/json")
        .body("{ \"result\": \"error\" }")
        .build())
    }    

}
