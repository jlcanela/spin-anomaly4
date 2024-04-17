use jsonwebtoken::{decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::jwks::load_jwks;
use jsonwebtoken::jwk::JwkSet;

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

pub async fn has_role(jwks: &String, audience: &String, id_token: &String, role: &String) -> Result<bool, String> {
    let roles = roles(jwks, &audience, &id_token).await?;
    Ok(roles.contains(&role))
}

pub async fn jwks(domain: &String) -> Result<String, String> {
    load_jwks(domain).await
}

pub async fn roles(jwks_string: &String, audience: &String, id_token: &String) -> Result<Vec<String>, String> {

    let jwks = serde_json::from_str::<JwkSet>(&jwks_string).map_err(|e| e.to_string())?;

    let header = decode_header(id_token).map_err(|e| format!("roles: {:?} 3", e.to_string()))?;
    let Some(kid) = header.kid else {
        // return Err("...".into())
        panic!("should have a key id");
    };

    let jwk = jwks.find(&kid).ok_or("no matching JWK fonud for the given kid")?;
 
    let decoding_key = match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa) => DecodingKey::from_rsa_components(&rsa.n, &rsa.e),
        _ => unreachable!("algorithm should be RSA")
    }.map_err(|e| format!("roles: {:?} 4", e.to_string()))?;

    let validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&vec![audience]);
        validation
    };

    let decoded_token = decode::<Claims>(&id_token, &decoding_key, &validation).map_err(|e| format!("roles: {:?} 5", e.to_string()))?;
    Ok(decoded_token.claims.anomaly4namespace.roles)
}

