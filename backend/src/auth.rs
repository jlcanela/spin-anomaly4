use api::jwks;
use jsonwebtoken::jwk::JwkSet;
use reqwest;
use spin_sdk::{http::{Params, Request}, variables};
use anyhow::Result;
use jsonwebtoken::{decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Roles {
    roles: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   #[serde(rename(deserialize = "https://anomaly4.mygameapp.net/jwt/claims"))]
   anomaly4namespace: Roles
}

async fn load_jwks(domain: &String) -> Result<JwkSet> {
    let url = format!("https://{}/.well-known/jwks.json", domain);
    
    let content = reqwest::get(url)
    .await?
    .text().await?;

    serde_json::from_str(&content).map_err(|e| e.into())
} 

pub async fn has_access(req: Request, _params: Params) -> Result<bool> {
    // let id_token = req.header("Authorization")
    //     .ok_or(anyhow::Error::msg("No Authorization header"))?
    //     .as_str()
    //     .ok_or(anyhow::Error::msg("Authorization header is not a string"))?;

    // let auth0_domain = variables::get("auth0_domain")?;
    // let audience = variables::get("auth0_client_id")?;

    // let jwks = load_jwks(&auth0_domain).await?;

    // let header = decode_header(id_token).unwrap();
    // let kid = header.kid.ok_or(anyhow::Error::msg("should have a key id"))?;
    // let jwk = jwks.find(&kid).ok_or(anyhow::Error::msg("no matching JWK fonud for the given kid"))?;


    // let decoding_key = match &jwk.algorithm {
    //     AlgorithmParameters::RSA(rsa) => DecodingKey::from_rsa_components(&rsa.n, &rsa.e).expect("Should have been able to create the key"),
    //     _ => unreachable!("algorithm should be RSA")
    // };

    // let validation = {
    //     let mut validation = Validation::new(Algorithm::RS256);
    //     validation.set_audience(&vec![audience]);
    //     validation
    // };

    // let decoded_token = decode::<Claims>(&id_token, &decoding_key, &validation).expect("Should have been able to decode the token");
    // // decoded_token.claims.anomaly4namespace.roles;

    Ok(true)
}
