use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use surrealdb::Error;
use surrealdb::sql::Thing;

use crate::models::album::{Album, AlbumReturn, AlbumWrapper};
use crate::models::directory::DirectoryWrapper;
use crate::utils;
use crate::utils::auth::string_to_thing;

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
            owner: Some(user_id)
        }
    ).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::CONFLICT, err.to_string())
        }
    };

    // TODO: Create empty parent directory

    // Return new album ID
    let id = match utils::get_id::get_album_id(payload.path).await {
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
pub async fn query_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Thing>) -> (StatusCode, Result<Json<AlbumReturn>, String>) {

    // Authenticate with JWT & extract metadata
    let _: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, Err(err.to_string()))
    };

    let result: Result<AlbumWrapper, Error> = crate::DB.select(("album", payload.id)).await;

    let album: AlbumWrapper = match result {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()));
        }
    };

    let mut dir_result = match crate::DB.query("SELECT * FROM directory WHERE album = $album AND parent = NONE")
        .bind(("album", string_to_thing(album.id.clone()))).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()));
        }
    };

    let root = match dir_result.take::<Option<DirectoryWrapper>>(0) {
        Ok(val) => match val {
            Some(val) => val,
            None => {
                crate::DB.invalidate();
                return (StatusCode::INTERNAL_SERVER_ERROR, Err("Database error".to_string()));
            }
        },
        Err(err) => {
            crate::DB.invalidate();
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()));
        }
    };

    crate::DB.invalidate();
    (StatusCode::OK, Ok(Json::from(AlbumReturn { album, root })))

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