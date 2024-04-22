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

impl Star {
    pub fn check(self: &Self, order: &Order) ->  Result<(), OrderFailed>  {
        
        match order {
            Order::Produce { .. } => Ok(()), 
            Order::Loot { .. } => if self.dev == 0 { 
                Err(OrderFailed::NotEnoughDev)
            } else { Ok(()) },
            Order::Develop { .. } => if self.dev >= self.dev_max { 
                Err(OrderFailed::TooMuchDev)
            } else if self.shuttles < 3 {
                Err(OrderFailed::NotEnoughShuttles)
            } else {
                Ok(())
            },
            Order::Colonize { .. }  => if self.dev_max > 0 {
                Err(OrderFailed::DevShouldBeZero)
            } else if self.shuttles < 3 {
                Err(OrderFailed::NotEnoughShuttles)
            } else {
                Ok(())
            },
            Order::Move { .. }  => Ok(()) 
        }
    }
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

    pub fn cost(self: &Self) -> i32 {
        match self {
            Order::Produce { star_id: _ } => 8,
            Order::Loot { star_id: _ } => 1,
            Order::Develop { star_id: _ } => 1,
            Order::Colonize { star_id: _ } => 8,
            Order::Move { star_id: _, dst_id: _, nb: _ } => 1,
        }
    }

    pub fn check(self: &Self, player: &Player) -> Result<(), OrderFailed> {
        if player.points < self.cost() {
             Err(OrderFailed::NotEnoughPoints) 
        } else {
            Ok(())
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderFailed {
    ServiceFailure(String, String),
    NotEnoughPoints,
    NotEnoughShuttles,
    NotEnoughDev,
    TooMuchDev,
    DevShouldBeZero,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderResult {
    Produce { name: String, produced_shuttles: i32, points: i32 },
    Loot { name: String, produced_shuttles: i32, points: i32 },
    Develop { name: String, consumed_shuttles: i32, new_dev: i32, points: i32 },
    Colonize { name: String, consumed_shuttles: i32, new_dev: i32, points: i32 },
    Move { name_source: String, name_destination: String, moved_shuttles: i32, points: i32 },
    Attack { name_source: String, name_destination: String, attacking_shuttles: i32, lost_shuttles: i32, destroyed_shuttles: i32, points: i32 },
    OrderFailed(OrderFailed)
}
