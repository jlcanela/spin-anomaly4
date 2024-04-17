use jsonwebtoken::{decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::jwks::load_jwks;

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

pub async fn roles(domain: &String, audience: &String, id_token: &String) {

    let jwks = load_jwks(domain).await;

    let header = decode_header(id_token).unwrap();
    let Some(kid) = header.kid else {
        // return Err("...".into())
        panic!("should have a key id");
    };

    let Some(jwk) = jwks.find(&kid) else {
        // return Err("...".into())
        panic!("no matching JWK fonud for the given kid");
    };  

    let decoding_key = match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa) => DecodingKey::from_rsa_components(&rsa.n, &rsa.e).expect("Should have been able to create the key"),
        _ => unreachable!("algorithm should be RSA")
    };

    let validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&vec![audience]);
        validation
    };

    let decoded_token = decode::<Claims>(&id_token, &decoding_key, &validation).expect("Should have been able to decode the token");
    println!("token roles {:?}", decoded_token.claims.anomaly4namespace.roles);
}

