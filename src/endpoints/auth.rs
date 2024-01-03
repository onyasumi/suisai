use crate::models;
use crate::utils;

use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use surrealdb::opt::auth::Jwt;
use surrealdb::opt::auth::Scope;
use surrealdb::sql::Thing;
use crate::endpoints::axum_error::{AxumError, IntoAxumError};

#[debug_handler]
pub async fn create_user(Json(payload): Json<models::auth::User>) -> Result<(StatusCode, String), AxumError> {

    let token: Jwt = crate::DB.signup(Scope {
        namespace: "test",
        database: "test",
        scope: "account",
        params: payload,
    }).await.or_error(StatusCode::CONFLICT)?;

    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::CREATED, token.into_insecure_token()))

}

#[debug_handler]
pub async fn login(Json(payload): Json<models::auth::User>) -> Result<(StatusCode, String), AxumError> {

    let token: Jwt = crate::DB.signin(Scope {
        namespace: "test",
        database: "test",
        scope: "account",
        params: payload,
    }).await.or_error(StatusCode::UNAUTHORIZED)?;

    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::OK, token.into_insecure_token()))

}

#[debug_handler]
pub async fn update_credentials(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<models::auth::User>) -> Result<(StatusCode, String), AxumError> {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // TODO: invalidate database on 500
    // Change email and/or password
    if !payload.email.is_empty() {
        crate::DB.query("UPDATE $id SET email = $new_email")
            .bind(("id", user_id.clone()))
            .bind(("new_email", payload.email)).await.or_500()?;
    }

    if !payload.password.is_empty() {
        crate::DB.query("UPDATE $id SET password = crypto::argon2::generate($new_password)")
            .bind(("id", user_id))
            .bind(("new_password", payload.password)).await.or_500()?;
    }

    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::CREATED, "meow".to_string()))

}