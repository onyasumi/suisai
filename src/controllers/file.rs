use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use surrealdb::Error;
use surrealdb::sql::Thing;

use crate::models::file::{File, FileWrapper};
use crate::utils;
use crate::utils::get_id::get_file_id;


#[debug_handler]
pub async fn create_file(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<File>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Create file
    let _: File = match crate::DB.create("file").content(payload.clone()).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::CONFLICT, err.to_string())
        }
    };

    // Return new file ID
    let id = match get_file_id(payload).await {
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
pub async fn delete_file(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<FileWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    let result: Result<File, Error> = crate::DB.delete(("file", payload.id)).await;

    crate::DB.invalidate();

    return match result {
        Ok(_) => (StatusCode::OK, "meow".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

}

#[debug_handler]
pub async fn relocate_file(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<FileWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Set new values for parent/album
    match crate::DB.query("UPDATE $id SET parent = $new_parent AND album = $new_album")
        .bind(("id", payload.id))
        .bind(("new_parent", payload.file.directory.parent))
        .bind(("new_album", payload.file.directory.album)).await {
        Ok(_) => (),
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    crate::DB.invalidate();
    (StatusCode::OK, "meow".to_string())

}
