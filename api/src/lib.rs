pub mod jwks;
pub mod jwt; 

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

