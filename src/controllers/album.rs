use crate::models;
use crate::utils;

use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use surrealdb::sql::Thing;

// Maybe return

#[debug_handler]
pub async fn create_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<models::album::Album>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Create album
    let _: models::album::Album = match crate::DB.create("album").content(
        models::album::Album {
            label: payload.label,
            path: payload.path,
            children: payload.children,
            owner: Some(user_id)
        }
    ).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::CONFLICT, err.to_string())
        }
    };

    crate::DB.invalidate();
    (StatusCode::OK, "meow".to_string())

}