use jsonwebtoken::jwk::JwkSet;

pub async fn load_jwks(domain: &String) -> Result<JwkSet, String> {
    let url = format!("https://{}/.well-known/jwks.json", domain);
    
    let content = reqwest::get(url)
    .await.map_err(|e| e.to_string())?
    .text().await.map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())?
} 

