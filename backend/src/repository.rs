use std::time::SystemTime;

use api::{Order, OrderFailed, OrderResult, Player, Situation, Star};
use spin_sdk::key_value::Store;

use OrderFailed::ServiceFailure;

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

        Ok(())
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
        Ok(res)
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
        Ok(json)
    }

    pub fn update_star<F>(self: &Self, star_id: i32, order: &Order, f: F) -> Result<Star, OrderFailed>
    where F: Fn(&mut Star) -> ()
    {
        let store = Store::open_default().map_err(|e| ServiceFailure("Data not accessible".to_string(), e.to_string()))?;
        let mut stars = self.load_from_kv::<Vec<Star>>("stars".to_string(), &store).map_err(|s| ServiceFailure("Data not accessible (stars)".to_string(), s))?;
        let star_index = stars.iter().position(|s| s.id == star_id).ok_or(ServiceFailure("Star not found".to_string(), star_id.to_string()))?;
        let star = &mut stars[star_index];
        star.check(order)?;
        f(star);
        let stars_str = serde_json::to_string(&stars).map_err(|s| ServiceFailure("Saving Stars Failed".to_string(), format!("Serialization|{}", s)))?;
        store.set("stars", stars_str.as_bytes()).map_err(|s| ServiceFailure("Saving Stars Failed".to_string(), format!("Store|{}", s)))?;
        Ok(stars[star_index].clone())
    }

    
    pub fn produce(self: &Self, star_id: i32, player: &mut Player, ordr: &Order) -> Result<OrderResult, OrderFailed> {
        tracing::info!("produce: {}", star_id);
        let cost_point = ordr.cost();
        let produce_cmd = |star: &mut Star|{
            star.shuttles += star.dev; 
        };
        
        let star = self.update_star(star_id, ordr,  produce_cmd)?;
        player.points -= cost_point;
        Ok(OrderResult::Produce { name: star.name.clone(), produced_shuttles: star.dev, points: cost_point})
    }
    
    pub fn loot(self: &Self, star_id: i32, player: &mut Player, ordr: &Order) -> Result<OrderResult, OrderFailed> {
        tracing::info!("loot: {}", star_id);
        let cost_point = ordr.cost();
        let cost_dev = 1;
        let amount_shuttles = 3;
        
        let loot_cmd = |star: &mut Star|{
            star.shuttles += amount_shuttles;
            star.dev -= cost_dev;
        };
        
        let star = self.update_star(star_id, ordr, loot_cmd)?;
        player.points -= cost_point;
        Ok(OrderResult::Loot{ name: star.name.clone(), produced_shuttles: amount_shuttles, points: cost_point})
    }
    
    pub fn develop(self: &Self, star_id: i32, player: &mut Player, ordr: &Order) -> Result<OrderResult, OrderFailed> {
        tracing::info!("develop: {}", star_id);
        let cost_point = ordr.cost();
        let cost_shuttles = 3;
        let amount_dev = 1;
        
        let loot_cmd = |star: &mut Star|{
            star.shuttles -= cost_shuttles;
            star.dev += amount_dev;
        };
        
        let star = self.update_star(star_id, ordr, loot_cmd)?;
        player.points -= cost_point;
        
        Ok(OrderResult::Develop { name: star.name.clone(), consumed_shuttles: cost_shuttles, new_dev: star.dev, points: cost_point })
    }
    
    pub fn colonize(self: &Self, star_id: i32, player: &mut Player, ordr: &Order) -> Result<OrderResult, OrderFailed> {
        tracing::info!("develop: {}", star_id);
        let cost_point = ordr.cost();
        let cost_shuttles = 3;
        let amount_dev = 1;
        
        let colonize_cmd = |star: &mut Star|{
            star.shuttles -= cost_shuttles;
            star.dev = amount_dev;
        };

        let star = self.update_star(star_id, ordr, colonize_cmd)?;
        player.points -= cost_point;
        Ok(OrderResult::Colonize { name: star.name.clone(), consumed_shuttles: cost_shuttles, new_dev: star.dev, points: cost_point })
    }
    
    pub fn order(self: &Self, order_str: String, player_id: i32) -> Result<OrderResult, OrderFailed> {
        let ordr = serde_json::from_str::<Order>(&order_str).map_err(|e|ServiceFailure("Invalid Order Format".to_string(), e.to_string()))?;
        let store = Store::open_default().map_err(|e| ServiceFailure("Data not accessible".to_string(), e.to_string()))?;
        let mut players = self.load_from_kv::<Vec<Player>>("players".to_string(), &store).map_err(|s| ServiceFailure("Data not accessible (players)".to_string(), s))?;
        let player_index = players.iter().position(|p| p.id == player_id).ok_or(ServiceFailure("Player not found".to_string(), player_id.to_string()))?;
        let player = &mut players[player_index];
        let check = ordr.check(player);
        tracing::info!("order: {:?}, player: {:?}, check: {:?}", ordr, player, check);
        let _ = check?;
        
        let res = match ordr {
            Order::Produce { star_id }=> self.produce(star_id, player, &ordr),
            Order::Loot { star_id } => self.loot(star_id, player, &ordr),
            Order::Develop { star_id } => self.develop(star_id, player, &ordr),
            Order::Colonize { star_id } => self.colonize(star_id, player, &ordr),
            Order::Move { star_id, dst_id, nb } => {
                tracing::info!("move: {} to {} shuttles {}", star_id, dst_id, nb);
                //     Attack { name_source: String, name_destination: String, attacking_shuttles: i32, lost_shuttles: i32, destroyed_shuttles: i32, points: i32 },
                Ok(OrderResult::Move { name_source: "".to_string(), name_destination: "".to_string(), moved_shuttles: 0, points: 0 })
            }
        };

        let players_str = serde_json::to_string(&players).map_err(|s| ServiceFailure("Saving Player Failed".to_string(), format!("Serialization|{}", s)))?;
        store.set("players", players_str.as_bytes()).map_err(|s| ServiceFailure("Saving Player Failed".to_string(), format!("Store|{}", s)))?;
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