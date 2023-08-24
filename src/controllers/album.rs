use crate::models;
use crate::utils;

use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use surrealdb::Error;
use surrealdb::sql::Thing;
use crate::models::album::{Album, AlbumWrapper};


#[debug_handler]
pub async fn create_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Create album
    let _: Album = match crate::DB.create("album").content(
        Album {
            label: payload.label,
            path: payload.path.clone(),
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

    // Return new album ID
    let id = match utils::album::album_id_from_path(payload.path).await {
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
pub async fn rename_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<AlbumWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    match crate::DB.query("UPDATE $id SET label = $new_label")
        .bind(("id", payload.id))
        .bind(("new_label", payload.album.label)).await {
            Ok(_) => (),
            Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    crate::DB.invalidate();

    (StatusCode::OK, "meow".to_string())

}


#[debug_handler]
pub async fn get_albums(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(_payload): Json<AlbumWrapper>) -> (StatusCode, Result<Json<Vec<AlbumWrapper>>, String>) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, Err(err.to_string()))
    };

    match crate::DB.select::<Vec<AlbumWrapper>>("album").await {
        Ok(vec) => {
            crate::DB.invalidate();
            (StatusCode::OK, Ok(Json(vec)))
        },
        Err(err) => {
            crate::DB.invalidate();
            (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
        }
    }

}


#[debug_handler]
pub async fn delete_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<AlbumWrapper>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // TODO: Delete album folder(s) first


    // Delete album from DB and return result
    let result: Result<Album, Error> = crate::DB.delete(("album", payload.id)).await;

    crate::DB.invalidate();

    return match result {
        Ok(_) => (StatusCode::OK, "meow".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

}