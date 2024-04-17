pub async fn load_jwks(domain: &String) -> Result<String, String> {
    let url = format!("https://{}/.well-known/jwks.json", domain);
    
    let content = reqwest::get(url)
    .await.map_err(|e| format!("load_jwks:{:?} 1", e.to_string()))?
    .text()
    .await.map_err(|e| e.to_string())?;

    Ok(content)
} 

