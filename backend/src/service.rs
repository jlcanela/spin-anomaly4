use crate::utils::json;

use self::{repository::{load_game, World}, utils::no_content};

use super::*;

use anyhow::Result;
use api::{Order, Situation, Star, WebConfig};
use spin_sdk::http::Params;

use OrderFailed::ServiceFailure;

pub fn handle_config(_req: Request, _parms: Params) -> Result<impl IntoResponse> {

    let auth0_client_id = variables::get("auth0_client_id")?;
    let auth0_domain = variables::get("auth0_domain")?;
    
    let config = WebConfig {
        client_id: auth0_client_id,
        auth0_domain: auth0_domain,
    };

    json(&config)
}

pub fn handle_situation(_req: Request, _parms: Params) -> Result<impl IntoResponse> {

    let world = repository::load_game()?;
    let situation = Situation {
        player: world.players[1].clone(),
        stars: world.stars,
        radar: world.radar,
    };

    json(&situation)
}

pub async fn handle_init(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let auth = auth::has_access(req, _params).await;
    _ = repository::init_game()?;
    tracing::info!("init game");
    
    Ok(no_content())
}

pub fn handle_clear(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    
    _ = repository::clear_game()?;
    tracing::info!("clear game");
    
    Ok(no_content())
}

fn get_star<'a>(star_id: &i32, stars: &'a mut Vec<Star>) -> Result<&'a mut Star, OrderFailed> {
    let star_index = stars.iter().position(|s| s.id == *star_id).ok_or(ServiceFailure("Star not found".to_string(), star_id.to_string()))?;
    let star: &mut Star = &mut stars[star_index];
    Ok(star)
}

pub fn produce(star_id: &i32, order: &Order, player_index: usize, world: &mut World) -> Result<OrderResult, OrderFailed> {
    let cost_points = order.cost();
    let player = &mut world.players[player_index];
    let star: &mut Star = get_star(star_id, &mut world.stars)?;
    let _ = star.check(order)?;
    star.shuttles += star.dev; 
    player.points -= cost_points;
    Ok(OrderResult::Produce { name: star.name.clone(), produced_shuttles: star.dev, points: cost_points})
}

pub fn loot(star_id: &i32, order: &Order, player_index: usize, world: &mut World) -> Result<OrderResult, OrderFailed> {
    let cost_point = order.cost();
    let amount_shuttles = 3;
    let cost_dev = 1;
    let player = &mut world.players[player_index];
    let star = get_star(star_id, &mut world.stars)?;
    let _ = star.check(order)?;
    star.shuttles += amount_shuttles; 
    star.dev_max -= cost_dev;
    if star.dev > star.dev_max {
        star.dev = star.dev_max;
    }
    player.points -= cost_point;
    Ok(OrderResult::Produce { name: star.name.clone(), produced_shuttles: amount_shuttles, points: cost_point})
}

pub fn develop(star_id: &i32, order: &Order, player_index: usize, world: &mut World) -> Result<OrderResult, OrderFailed> {
    let cost_point = order.cost();
    let cost_shuttles = 3;
    let amount_dev = 1;
    let player = &mut world.players[player_index];
    let star = get_star(star_id, &mut world.stars)?;
    let _ = star.check(order)?;
    star.shuttles -= cost_shuttles;
    star.dev += amount_dev;
    player.points -= cost_point;
    Ok(OrderResult::Develop { name: star.name.clone(), consumed_shuttles: cost_shuttles, new_dev: star.dev, points: cost_point })
}

pub fn colonize(star_id: &i32, order: &Order, player_index: usize, world: &mut World) -> Result<OrderResult, OrderFailed> {
    let cost_point = order.cost();
    let cost_shuttles = 3;
    let amount_dev = 1;
    let player = &mut world.players[player_index];
    let star = get_star(star_id, &mut world.stars)?;
    let _ = star.check(order)?;
    star.shuttles -= cost_shuttles;
    star.dev = amount_dev;
    star.dev_max = amount_dev;
    player.points -= cost_point;
    Ok(OrderResult::Colonize { name: star.name.clone(), consumed_shuttles: cost_shuttles, new_dev: star.dev, points: cost_point })
}


pub fn execute_order(order: &Order, player_index: usize, world: &mut World) -> Result<OrderResult, OrderFailed> {
    let result = match order {
        Order::Produce { star_id } => produce(star_id, order, player_index, world)?,
        Order::Loot { star_id } => loot(star_id, order, player_index, world)?,
        Order::Develop { star_id } => develop(star_id, order, player_index, world)?,
        Order::Colonize { star_id } => colonize(star_id, order, player_index, world)?,
        _ => Err(ServiceFailure("order not implemented".to_string(), "".to_string()))?
        // Order::Move { star_id, dst_id, nb } => {
        //     //     Attack { name_source: String, name_destination: String, attacking_shuttles: i32, lost_shuttles: i32, destroyed_shuttles: i32, points: i32 },
        //     Ok(OrderResult::Move { name_source: "".to_string(), name_destination: "".to_string(), moved_shuttles: 0, points: 0 })
    };
    let _ = repository::save_game(&world).map_err(|e| OrderFailed::ServiceFailure("INTERNAL_ERROR".to_string(), e.to_string()))?;
    Ok(result)
}

pub fn handle_order(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let order = serde_json::from_slice::<Order>(req.body())?;
    let player_id: usize = 1;

    let mut world: World = load_game()?;
    let player_index = world.players.iter().position(|p| p.id == player_id as i32)
        .ok_or(anyhow::Error::msg("Player not found"))?;    
    let result = execute_order(&order, player_index, &mut world);

    let reply = match result {
        Ok(r) => r,
        Err(failure) => OrderResult::OrderFailed(failure)
    };
    Ok(json(&reply))
}
