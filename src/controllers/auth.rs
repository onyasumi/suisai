use crate::models;
use crate::utils;

use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use serde_json::to_string;
use surrealdb::opt::auth::Jwt;
use surrealdb::opt::auth::Scope;
use surrealdb::sql::Thing;

#[debug_handler]
pub async fn create_user(Json(payload): Json<models::auth::User>) -> (StatusCode, String) {

    let token: Jwt = match crate::DB.signup(Scope {
        namespace: "test",
        database: "test",
        scope: "account",
        params: payload,
    }).await {
        Ok(val) => val,
        Err(err) => return (StatusCode::CONFLICT, err.to_string())
    };

    if let Err(e) = crate::DB.invalidate().await {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    } else {
        (StatusCode::CREATED, to_string(&token).unwrap())
    }

}

#[debug_handler]
pub async fn login(Json(payload): Json<models::auth::User>) -> (StatusCode, String) {

    let token: Jwt = match crate::DB.signin(Scope {
        namespace: "test",
        database: "test",
        scope: "account",
        params: payload,
    }).await {
        Ok(val) => val,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    if let Err(e) = crate::DB.invalidate().await {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    } else {
        (StatusCode::CREATED, to_string(&token).unwrap())
    }

}

#[debug_handler]
pub async fn update_credentials(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<models::auth::User>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Change email and/or password
    if !payload.email.is_empty() {
        crate::DB.query("UPDATE $id SET email = $new_email")
            .bind(("id", user_id.clone()))
            .bind(("new_email", payload.email)).await.unwrap();
    }

    if !payload.password.is_empty() {
        crate::DB.query("UPDATE $id SET password = crypto::argon2::generate($new_password)")
            .bind(("id", user_id))
            .bind(("new_password", payload.password)).await.unwrap();
    }

    if let Err(e) = crate::DB.invalidate().await {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    } else {
        (StatusCode::CREATED, "meow".to_string())
    }

}