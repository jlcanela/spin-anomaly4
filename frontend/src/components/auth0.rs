use leptos::*;
use leptos_oidc::{Auth, AuthParameters};
use serde_json;

use api::{ConfigError, WebConfig};
use crate::api::api::Api;


fn make_auth_params(client_id: String, auth0_domain: String, base_url: String) -> AuthParameters {

    let redirect_uri = format!("{}/game", base_url);
    let post_logout_redirect_uri = format!("{}/", base_url);

    AuthParameters {
        auth_endpoint: format!("https://{}/authorize", auth0_domain),
        token_endpoint: format!("https://{}/oauth/token", auth0_domain),
        logout_endpoint: format!("https://{}/oidc/logout", auth0_domain),
        client_id: client_id,
        redirect_uri: redirect_uri,
        post_logout_redirect_uri: post_logout_redirect_uri,
        scope: Some("openid offline_access".to_string()),
    }
}

async fn load_config(url: String) -> Result<WebConfig, ConfigError> {
    
    let client = reqwest::Client::new();
    let res = client.post(url)
    .body("the exact body that is sent")
    .send()
    .await.map_err(|_| ConfigError::FetchError)?; // issue with request
    
    let as_text = res.text().await.map_err(|_| ConfigError::ParseError)?; // Issue with response

    let config: WebConfig = serde_json::from_str(&as_text).map_err(|_| ConfigError::ParseError)?;

    Ok(config)
}

pub async fn auth(base_url: String, config_url: String) -> bool {
    let config = load_config(config_url).await;
    if config.is_ok() {
        let c = config.unwrap();
        let auth_parameters = make_auth_params(c.client_id.clone(), c.auth0_domain.clone(), base_url.clone());
        let _auth = Auth::init(auth_parameters);

        provide_context(Api::new(base_url, c));

        return true;
    } else {
        return false;
    }
}
