use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use surrealdb::sql::Thing;

use crate::models::directory::{Directory, DirectoryWrapper};
use crate::utils;
use crate::utils::directory::delete_recursive;
use crate::utils::get_id::{get_directory_id};

#[debug_handler]
pub async fn create_directory(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Directory>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Create directory
    let _: Directory = match crate::DB.create("directory").content(payload.clone()).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::CONFLICT, err.to_string())
        }
    };

    // Return new directory ID
    let id = match get_directory_id(payload).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    };

    crate::DB.invalidate();
    (StatusCode::OK, id)

}


#[debug_handler]
pub async fn delete_directory(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<DirectoryWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    match delete_recursive(payload.id).await {
        Ok(_) => (),
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    crate::DB.invalidate();
    (StatusCode::OK, "meow".to_string())

}