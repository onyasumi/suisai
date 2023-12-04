use crate::models::auth;

use base64::{Engine as _, engine::general_purpose};
use anyhow::Result;
use std::str;
use surrealdb::sql::Thing;

pub fn extract_jwt_header(token: &str) -> Result<auth::JwtPayload> {

    // Extract payload and decode base64 to json
    let jwt_split: Vec<&str> = token.split('.').collect();
    let jwt_payload_ascii = general_purpose::STANDARD_NO_PAD.decode(jwt_split[1])?;

    // Deserialize json and return 
    Ok(serde_json::from_str(
        // Convert ASCII vector to &str
        str::from_utf8(&jwt_payload_ascii)?
    )?)

}

pub fn string_to_thing(id_str: String) -> Thing {

    let id_str_split: Vec<&str> = id_str.split(':').collect();

    Thing::from((id_str_split[0], id_str_split[1]))

}

pub async fn authenticate(token: &str) -> Result<Thing> {

    // Authenticate with JWT & extract metadata
    crate::DB.authenticate(token).await?;
    
    // Extract header from DB
    let jwt_payload: auth::JwtPayload = extract_jwt_header(token)?;
    crate::DB.use_ns(jwt_payload.ns).use_db(jwt_payload.db).await?;

    // Get the user ID and return as a `Thing`
    Ok(string_to_thing(jwt_payload.id))

}