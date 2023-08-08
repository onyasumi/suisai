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
pub async fn create_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<models::auth::User>) -> (StatusCode, String) {





    (StatusCode::IM_A_TEAPOT, "meow".to_string())

}