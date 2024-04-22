use std::time::SystemTime;

use api::{Player, Star};
use spin_sdk::key_value::Store;

pub struct World {
    pub stars: Vec<Star>,
    pub radar: Vec<Star>,
    pub players: Vec<Player>,
}

const STARS_KEY: &str = "stars";
const RADAR_KEY: &str = "radar";
const PLAYERS_KEY: &str = "players";

pub fn init_game() -> Result<(), anyhow::Error> {
    let world = World {
        stars: Fixtures::stars(),
        radar: Fixtures::radar(),
        players: Fixtures::players(),
    };

    let store = Store::open_default()?;
    store.set_json(STARS_KEY, &world.stars)?;
    store.set_json(RADAR_KEY, &world.radar)?;
    store.set_json(PLAYERS_KEY, &world.players)?;
    Ok(())
}

pub fn clear_game() -> Result<(), anyhow::Error> {
    let store = Store::open_default()?;
    let _ = store.delete(STARS_KEY)?;
    let _ = store.delete(RADAR_KEY)?;
    let _ = store.delete(PLAYERS_KEY)?;
    Ok(())
}

pub fn load_game() -> Result<World, anyhow::Error> {        
    let store = Store::open_default()?;
    let stars: Vec<Star> = store.get_json(STARS_KEY)?
        .ok_or(anyhow::Error::msg("Stars not found"))?;
    let radar: Vec<Star> = store.get_json(RADAR_KEY)?
        .ok_or(anyhow::Error::msg("Radar not found"))?;
    let players: Vec<Player> = store.get_json(PLAYERS_KEY)?
        .ok_or(anyhow::Error::msg("Players not found"))?;
    Ok(World { stars, radar, players })
}

pub fn save_game(world: &World) -> Result<(), anyhow::Error> {        
    let store = Store::open_default()?;
    store.set_json(STARS_KEY, &world.stars)?;
    store.set_json(RADAR_KEY, &world.radar)?;
    store.set_json(PLAYERS_KEY, &world.players)?;
    Ok(())
}

pub struct Fixtures {
}

impl Fixtures {

    pub fn stars() -> Vec<Star> {
        vec!(
            Star { id: 0, name: "Rataxi".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 1, name: "Rogafo".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 2, name: "Rikidi".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 3, name: "Naove".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 4, name: "Gimani".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 5, name: "Tatufo".to_string(), owner: "Player".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        )
    }
    
    pub fn radar() -> Vec<Star> {
        vec!(
            Star { id: 6, name: "Godami".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 7, name: "Lulubi".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
            Star { id: 8, name: "Daraki".to_string(), owner: "Empire".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        )
    }
    
    pub fn players() -> Vec<Player> {
        vec!(
            Player { id: 0, sub: None, name: "Empire".to_string(), points: 0, last_update: SystemTime::now() },
            Player { id: 1, sub: Some("google-oauth2|112177133653097483983".to_string()), name: "Player".to_string(), points: 50, last_update: SystemTime::now() },
        )
    }

}