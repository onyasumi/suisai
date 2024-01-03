use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use surrealdb::sql::Thing;
use crate::endpoints::axum_error::{AxumError, IntoAxumError};

use crate::models::album::Album;
use crate::utils;

#[debug_handler]
pub async fn create_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> Result<(StatusCode, String), AxumError> {

    // Authenticate with JWT & extract metadata
    let user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // TODO: invalidate upon auth failure
    // Create album
    crate::DB.create::<Vec<Album>>("album").content(
        Album {
            label: payload.label,
            path: payload.path.clone(),
            owner: Some(user_id),
            id: None
        }
    ).await.or_error(StatusCode::CONFLICT)?;

    // TODO: invalidate upon auth failure
    // Return new album ID
    let id = utils::get_id::get_album_id(payload.path).await.or_500()?;

    crate::DB.invalidate().await.unwrap();
    Ok((StatusCode::OK, id))

}


#[debug_handler]
pub async fn rename_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> Result<(StatusCode, String), AxumError> {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // TODO: Invalidate auth upon failure
    crate::DB.query("UPDATE $id SET label = $new_label")
        .bind(("id", payload.id))
        .bind(("new_label", payload.label)).await.or_500()?;

    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::OK, "meow".to_string()))

}


#[debug_handler]
pub async fn get_albums(TypedHeader(header): TypedHeader<Authorization<Bearer>>) -> Result<(StatusCode, Json<Vec<Album>>), AxumError> {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // Get album data
    let album = crate::DB.select::<Vec<Album>>("album").await.or_500()?;
    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::OK, Json(album)))

}


#[debug_handler]
pub async fn delete_album(TypedHeader(header): TypedHeader<Authorization<Bearer>>, Json(payload): Json<Album>) -> Result<(StatusCode, String), AxumError> {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // Delete album from DB and return result
    let _result: Option<Album> = crate::DB.delete(("album", payload.id.unwrap())).await.or_500()?;

    crate::DB.invalidate().await.unwrap();

    Ok((StatusCode::OK, "meow".to_string()))

}