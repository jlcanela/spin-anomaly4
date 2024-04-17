pub mod jwks;
pub mod jwt; 

use std::time::SystemTime;

use chrono::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Star {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub shuttles: i32,
    pub dev: i32,
    pub dev_max: i32,
}


#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String, 
    pub points: i32,
    pub last_update: SystemTime,
    pub sub: Option<String>,
}

impl Default for Player {
    fn default() -> Self {
        let dt = DateTime::parse_from_rfc3339("2000-01-01T08:00:00-00:00").unwrap();
        let st = SystemTime::from(dt);
        Player { id: 0, name: "Empire".to_string(), points: 0, last_update: st , sub: None }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Situation {
    pub player: Player, 
    pub stars: Vec<Star>,
    pub radar: Vec<Star>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Order {
    Produce(i32),
    Loot(i32),
    Develop(i32),
    Colonize(i32),
    Move(i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigError {
    FetchError,
    ParseError,
}

impl ToString for ConfigError {
    fn to_string(&self) -> String {
        match self {
            ConfigError::FetchError => "Fetch error".to_string(),
            ConfigError::ParseError => "Parse error".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebConfig {
    pub client_id: String,
    pub auth0_domain: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub name: String,
    pub points: i32,
}

