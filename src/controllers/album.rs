use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use surrealdb::Error;
use surrealdb::sql::Thing;

use crate::models::album::Album;
use crate::utils;

#[debug_handler]
pub async fn create_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // Create album
    if let Err(e) = crate::DB.create::<Vec<Album>>("album").content(
        Album {
            label: payload.label,
            path: payload.path.clone(),
            owner: Some(user_id),
            id: None
        }
    ).await {
        crate::DB.invalidate().await.unwrap();
        return (StatusCode::CONFLICT, e.to_string())
    };

    // Return new album ID
    let id = match utils::get_id::get_album_id(payload.path).await {
        Ok(val) => val,
        Err(err) => {
            crate::DB.invalidate().await.unwrap();
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    };

    crate::DB.invalidate().await.unwrap();
    (StatusCode::OK, id)

}


#[debug_handler]
pub async fn rename_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    match crate::DB.query("UPDATE $id SET label = $new_label")
        .bind(("id", payload.id))
        .bind(("new_label", payload.label)).await {
            Ok(_) => (),
            Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    crate::DB.invalidate().await.unwrap();

    (StatusCode::OK, "meow".to_string())

}


#[debug_handler]
pub async fn get_albums(TypedHeader(header): TypedHeader<Authorization<Bearer>>) -> (StatusCode, Result<Json<Vec<Album>>, String>) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, Err(err.to_string()))
    };

    match crate::DB.select::<Vec<Album>>("album").await {
        Ok(vec) => {
            crate::DB.invalidate().await.unwrap();
            (StatusCode::OK, Ok(Json(vec)))
        },
        Err(err) => {
            crate::DB.invalidate().await.unwrap();
            (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()))
        }
    }

}


#[debug_handler]
pub async fn delete_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> (StatusCode, String) {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = match utils::auth::authenticate(header.token()).await {
        Ok(t) => t,
        Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string())
    };

    // TODO: Delete album folder(s) first


    // Delete album from DB and return result
    let result: Result<Option<Album>, Error> = crate::DB.delete(("album", payload.id.unwrap())).await;

    crate::DB.invalidate().await.unwrap();

    return match result {
        Ok(_) => (StatusCode::OK, "meow".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

}