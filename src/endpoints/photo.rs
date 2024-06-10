use axum::{debug_handler, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use surrealdb::sql::Thing;
use crate::endpoints::axum_error::{AxumError, IntoAxumError};

use crate::models::photo::Photo;
use crate::utils;

#[debug_handler]
pub async fn get_photos(TypedHeader(header): TypedHeader<Authorization<Bearer>>) -> Result<(StatusCode, Json<Vec<Photo>>), AxumError> {

    // Authenticate with JWT & extract metadata
    let _user_id: Thing = utils::auth::authenticate(header.token()).await.or_error(StatusCode::UNAUTHORIZED)?;

    // Get album data
    let album = crate::DB.select::<Vec<Photo>>("photos").await.or_500()?;
    crate::DB.invalidate().await.or_500()?;

    Ok((StatusCode::OK, Json(album)))

}