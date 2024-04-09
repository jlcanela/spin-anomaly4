use leptos::*;
use leptos_oidc::{Auth, AuthParameters};

use serde::{Deserialize, Serialize};

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

use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebConfig {
    pub client_id: String,
    pub auth0_domain: String,
}


fn make_auth_params(client_id: String, auth0_domain: String, base_url: String) -> AuthParameters {

    let redirect_uri = format!("{}/profile", base_url);
    let post_logout_redirect_uri = format!("{}/bye?destroy_session=true", base_url);

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

async fn auth(base_url: String, config_url: String) -> bool {
    let config = load_config(config_url).await;
    if config.is_ok() {
        let c = config.unwrap();
        let auth_parameters = make_auth_params(c.client_id.clone(), c.auth0_domain.clone(), base_url);
        let _auth = Auth::init(auth_parameters);
        return true;
    } else {
        return false;
    }
}

#[component]
pub fn MakeAuth0(
    base_url: String, 
    config_url: String,
    children: Box<dyn Fn() -> Fragment>, 
    #[prop(optional, into)] 
    loading: ViewFn) -> impl IntoView
    {
  
    let base_url = base_url.clone();
    let config = create_blocking_resource(|| (),  move |_|  { 
        auth(base_url.clone(), config_url.clone())
    });
    let view = store_value(children);

    view! {
        <Suspense fallback=loading>     
            {move || config.map(|auth| if *auth { view.with_value(|view| view().into_view()) } else { view! { <div>Error loading Auth</div>}.into_view() } )} 
        </Suspense>
    }
}
