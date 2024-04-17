use api::{Order, Situation, WebConfig};
use reqwest::Response;

#[derive(Clone)]
pub struct Api {
    pub base_url: String,
    pub config: WebConfig,
}


impl Api {
    pub fn new(base_url: String, config: WebConfig) -> Self {
        Self { base_url, config }
    }


    fn url_situation(&self) -> String {
        format!("{}/api/situation", self.base_url)
    }

    fn url_order(&self) -> String {
        format!("{}/api/order", self.base_url)
    }

    fn url_init(&self) -> String {
        format!("{}/api/init", self.base_url)
    }

    fn url_clear(&self) -> String {
        format!("{}/api/clear", self.base_url)
    }

    pub async fn situation(self: &Self) -> Option<Situation> {
        let res: Response = match reqwest::get(self.url_situation()).await {
            Ok(res) => res,
            _ => return None,
        };
        match res.json().await {
            Ok(situation) => situation,
            _ => None,
        }
    }

    async fn order(self: &Self, o: &Order) {
        let json = serde_json::to_string(o).unwrap();
        let client = reqwest::Client::new();

        let _response = client
            .post(self.url_order())
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await;
    }

    pub async fn init_game(self: &Self) {
        let client = reqwest::Client::new();

        let _response = client
            .post(self.url_init())
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await;
    }

    pub async fn clear_game(self: &Self) {
        let client = reqwest::Client::new();

        let _response = client
            .post(self.url_clear())
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await;
    }

    pub async fn produce_order(self: &Self, star_id: i32) {
        let cmd = Order::Produce(star_id);
        self.order(&cmd).await;
    }
}
