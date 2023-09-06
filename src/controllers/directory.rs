use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use surrealdb::sql::Thing;

use crate::models::directory::{Directory, DirectoryReturn, DirectoryWrapper};
use crate::models::file::FileWrapper;
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
pub async fn query_directory(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Thing>) -> (StatusCode, Result<Json<DirectoryWrapper>, String>) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, Err(err.to_string()))
    };

    let directory: DirectoryWrapper = match crate::DB.select(("directory", payload.id)).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
        }
    };

    crate::DB.invalidate();
    (StatusCode::OK, Ok(Json::from(directory)))

}

#[debug_handler]
pub async fn query_subdirectory(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Thing>) -> (StatusCode, Result<Json<DirectoryReturn>, String>) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, Err(err.to_string()))
    };

    let mut result = match crate::DB.query("SELECT * FROM directory WHERE parent = $parent_id")
        .query("SELECT * FROM file WHERE parent = $parent_id")
        .bind(("parent_id", payload)).await {
            Ok(val) => val,
            Err(err) => {
                crate::DB.invalidate();
                return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
            }
    };

    let directories: Vec<DirectoryWrapper> = match result.take(0) {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
        }
    };


    let files: Vec<FileWrapper> = match result.take(1) {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
        }
    };

    crate::DB.invalidate();
    (StatusCode::OK, Ok(Json::from(DirectoryReturn { directories, files })))

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


#[debug_handler]
pub async fn relocate_directory(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<DirectoryWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Set new values for parent/album
    match crate::DB.query("UPDATE $id SET parent = $new_parent AND album = $new_album")
        .bind(("id", payload.id))
        .bind(("new_parent", payload.directory.parent))
        .bind(("new_album", payload.directory.album)).await {
        Ok(_) => (),
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    crate::DB.invalidate();
    (StatusCode::OK, "meow".to_string())

}