use crate::models;
use crate::utils;

use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
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
        Err(_err) => return (StatusCode::CONFLICT, String::new())
    };

    crate::DB.invalidate();
    (StatusCode::CREATED, to_string(&token).unwrap())

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
        Err(_err) => return (StatusCode::UNAUTHORIZED, String::new())
    };

    crate::DB.invalidate();
    (StatusCode::OK, to_string(&token).unwrap())

}

#[debug_handler]
pub async fn update_credentials(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<models::auth::User>) -> (StatusCode, String) {

    // Authenticate with JWT
    match crate::DB.authenticate(header.token()).await {
        Ok(_) => (),
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    let jwt_payload: models::auth::JwtPayload = utils::auth::extract_jwt_header(header.token());

    // Change email and/or password
    crate::DB.use_ns(jwt_payload.ns).use_db(jwt_payload.db).await.unwrap();

    let user_id: Thing = utils::auth::string_to_thing(jwt_payload.id);

    if payload.email != "" {
        crate::DB.query("UPDATE $id SET email = $new_email")
            .bind(("id", user_id.clone()))
            .bind(("new_email", payload.email)).await.unwrap();
    }

    if payload.password != "" {
        crate::DB.query("UPDATE $id SET password = crypto::argon2::generate($new_password)")
            .bind(("id", user_id))
            .bind(("new_password", payload.password)).await.unwrap();
    }

    crate::DB.invalidate();
    (StatusCode::OK, "meow".to_string())

}