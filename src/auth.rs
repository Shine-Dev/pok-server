use std::error::Error;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{
    Validation, 
    Algorithm, 
    DecodingKey, 
    TokenData,
    decode_header, 
    decode
};
use crate::errors::ApiError;
use crate::environment;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,          
    iat: usize,
    aud: String, 
    iss: String,         
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyList {
    keys: Vec<KeyComponents>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyComponents {
    kid: String,
    n: String,
    e: String,
}

fn create_validation() -> Validation {
    let audience = environment::variables::expect_variable("AUDIENCE"); 
    let issuer = environment::variables::expect_variable("ISSUER"); 

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);
    validation.iss = Some(issuer);
    validation
}

fn get_kid(token: &str) -> Result<String, ApiError>{
    match decode_header(&token).map(|header| header.kid) {
        Ok(Some(kid)) => Ok(kid),
        _ => return Err(ApiError::KeyFetchFetchError),
    }
}

async fn fetch_keys() -> Result<KeyList, Box<dyn Error>>{
    let authority = environment::variables::expect_variable("AUTHORITY");
    let http_response = reqwest::get(&authority).await?;
    Ok(http_response.json::<KeyList>().await?)
}

pub async fn validate_token(token: &str) -> Result<TokenData<Claims>, ApiError> {

    let validation = create_validation();

    let key_list = fetch_keys().await
        .map_err(|_| ApiError::KeyFetchFetchError)?;

    let kid = get_kid(&token)?;

    let key_components = key_list.keys.iter()
        .find(|key| key.kid == kid)
        .ok_or(ApiError::KeyFetchFetchError)?;

    let decoding_key = DecodingKey::from_rsa_components(&key_components.n, &key_components.e);

    Ok(
        decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_| ApiError::AuthError)?
    )
}
