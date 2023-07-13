use crate::models::auth;

use base64::{Engine as _, engine::general_purpose};
use std::str;
use surrealdb::sql::Thing;

pub fn extract_jwt_header(token: &str) -> auth::JwtPayload {

    // Extract payload and decode base64
    let jwt_split: Vec<&str> = token.split(".").collect();
    let jwt_payload_ascii = general_purpose::STANDARD_NO_PAD.decode(jwt_split[1]).unwrap();

    // Deserialize and return
    serde_json::from_str(
        // Convert ASCII vector to &str
        str::from_utf8(&jwt_payload_ascii).unwrap()
    ).unwrap()

}

pub fn string_to_thing(id_str: String) -> Thing {

    let id_str_split: Vec<&str> = id_str.split(":").collect();

    Thing::from((id_str_split[0], id_str_split[1]))

}