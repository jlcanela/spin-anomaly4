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
