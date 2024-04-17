use std::time::SystemTime;

use anyhow::Ok;
use api::{Order, Player, Situation, Star};
use spin_sdk::key_value::Store;

pub struct Repository {    
}

impl Repository {
    pub fn new() -> Self {
        Self { }
    }
        
    pub fn init_game(self: &Self) -> Result<(), String> {
        let store = Store::open_default().map_err(|e| e.to_string())?;

        let stars = serde_json::to_string(&Fixtures::stars()).map_err(|e| e.to_string())?;
        store.set("stars", stars.as_bytes()).map_err(|e| e.to_string())?;

        let radar = serde_json::to_string(&Fixtures::radar()).map_err(|e| e.to_string())?;
        store.set("radar", radar.as_bytes()).map_err(|e| e.to_string())?;

        let players = serde_json::to_string(&Fixtures::players()).map_err(|e| e.to_string())?;
        store.set("players", players.as_bytes()).map_err(|e| e.to_string())?;

        Ok(()).map_err(|e| e.to_string())
    }

    pub fn clear_game(self: &Self) -> Result<(), String> {
        let store = Store::open_default().map_err(|e| e.to_string())?;
        store.delete("stars").map_err(|e| e.to_string())
    }

    fn load_from_kv<T: for<'de> serde::Deserialize<'de>>(self: &Self, name: String, store: &Store) -> Result<T, String> 
    {
        let binary = store.get(&name).map_err(|e| e.to_string())?;
        let buffer = binary.ok_or(format!("no game ({})", name))?;
        let str = String::from_utf8(buffer).map_err(|e| e.to_string())?;
        let res = serde_json::from_str::<T>(&str).map_err(|e| e.to_string())?;
        Ok(res).map_err(|e| e.to_string())
    }

    pub fn situation(self: &Self) -> Result<String, String> {

        let store = Store::open_default().map_err(|e| e.to_string())?;
        let stars = self.load_from_kv::<Vec<Star>>("stars".to_string(), &store)?;
        let radar = self.load_from_kv::<Vec<Star>>("radar".to_string(), &store)?;
        let players = self.load_from_kv::<Vec<Player>>("players".to_string(), &store)?;
                    
        let situation = Situation {
            player: players[1].clone(),
            stars: stars,
            radar: radar,
        };

        let json = serde_json::to_string(&situation).map_err(|e| e.to_string())?;
        Ok(json).map_err(|e| e.to_string())
    }

    pub fn update_star<F>(self: &Self, star_id: i32, f: F) -> Result<(), String>
    where F: Fn(&mut Star) -> ()
    {
        let store = Store::open_default().map_err(|e| e.to_string())?;
        let mut stars = self.load_from_kv::<Vec<Star>>("stars".to_string(), &store)?;
        let star_index = stars.iter().position(|s| s.id == star_id).ok_or(format!("no star {}", star_id))?;
        f(&mut stars[star_index]);
        let stars_str = serde_json::to_string(&stars).map_err(|e| e.to_string())?;
        store.set("stars", stars_str.as_bytes()).map_err(|e| e.to_string())?;
        Ok(()).map_err(|e| e.to_string())
    }

    pub fn update_player<F>(self: &Self, player_id: i32, f: F) -> Result<(), String>
    where F: Fn(&mut Player) -> ()
    {
            let store = Store::open_default().map_err(|e| e.to_string())?;
            let mut players = self.load_from_kv::<Vec<Player>>("players".to_string(), &store)?;
            let player_index = players.iter().position(|p| p.id == player_id).ok_or(format!("no player {}", player_id))?;
            f(&mut players[player_index]);
            let players_str = serde_json::to_string(&players).map_err(|e| e.to_string())?;
            store.set("players", players_str.as_bytes()).map_err(|e| e.to_string())?;
            Ok(()).map_err(|e| e.to_string())
    }

    pub fn produce(self: &Self, star_id: i32) -> Result<(), String> {
        tracing::info!("produce: {}", star_id);
        let produce_cmd = |star: &mut Star|star.shuttles += star.dev; 
        let _res = self.update_star(star_id, produce_cmd)?;
        let res = self.update_player(1, |player: &mut Player| player.points -= 8)?;
        Ok(res).map_err(|e| e.to_string())
    }

    pub fn loot(self: &Self, star_id: i32) -> Result<(), String> {
        tracing::info!("loot: {}", star_id);
        let loot_cmd = |star: &mut Star|{
            star.shuttles += 3;
            star.dev -= 1;
        };
        let _res = self.update_star(star_id, loot_cmd)?;
        let res = self.update_player(1, |player: &mut Player| player.points -= 1)?;
        Ok(res).map_err(|e| e.to_string())
    }

    pub fn develop(self: &Self, star_id: i32) -> Result<(), String> {
        tracing::info!("develop: {}", star_id);
        let loot_cmd = |star: &mut Star|{
            star.shuttles -= 3;
            star.dev += 1;
        };
        let _res = self.update_star(star_id, loot_cmd)?;
        let res = self.update_player(1, |player: &mut Player| player.points -= 8)?;

        Ok(res).map_err(|e| e.to_string())
    }

    pub fn colonize(self: &Self, star_id: i32) -> Result<(), String> {
        tracing::info!("develop: {}", star_id);
        let colonize_cmd = |star: &mut Star|{
            star.shuttles -= 3;
            star.dev = 1;
        };
        let _res = self.update_star(star_id, colonize_cmd)?;
        let res = self.update_player(1, |player: &mut Player| player.points -= 8)?;
        Ok(res).map_err(|e| e.to_string())
    }

    pub fn order(self: &Self, order_str: String) -> Result<(), String> {
        let order = serde_json::from_str::<Order>(&order_str).map_err(|e| e.to_string())?;
        tracing::info!("order: {:?}", order);
        let res = match order {
            Order::Produce { star_id }=> self.produce(star_id),
            Order::Loot { star_id } => self.loot(star_id),
            Order::Develop { star_id } => self.develop(star_id),
            Order::Colonize { star_id } => self.colonize(star_id),
            Order::Move { star_id, dst_id, nb } => {
                tracing::info!("move: {} to {} shuttles {}", star_id, dst_id, nb);
                Ok(()).map_err(|e| e.to_string())
            }
        };
        res         
    }

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