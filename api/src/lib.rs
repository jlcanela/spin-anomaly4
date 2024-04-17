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
    Produce { star_id: i32 },
    Loot { star_id: i32 },
    Develop { star_id: i32 },
    Colonize { star_id: i32 },
    Move { star_id: i32, dst_id: i32, nb: i32 },
}

impl Order {

    pub fn produce(star_id: i32) -> Self {
        Order::Produce { star_id }
    }   

    pub fn loot(star_id: i32) -> Self {
        Order::Loot { star_id }
    }

    pub fn develop(star_id: i32) -> Self {
        Order::Develop { star_id }
    }

    pub fn colonize(star_id: i32) -> Self {
        Order::Colonize { star_id }
    }

    pub fn with_star_id(self: &mut Self, star_id: i32) {
        match self {
            Order::Produce { star_id: _ } => { *self = Order::Produce { star_id } },
            Order::Loot { star_id: _ } => { *self = Order::Loot { star_id } },
            Order::Develop { star_id: _ } => { *self = Order::Develop { star_id } },
            Order::Colonize { star_id: _ } => { *self = Order::Colonize { star_id } },
            Order::Move { star_id: _, dst_id: _, nb: _ } => { *self = Order::Move { star_id, dst_id: 0, nb: 0 } },
        } 
    }
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

