mod endpoints;
pub(crate) mod utils;
pub(crate) mod models;

use axum::Router;
use axum::routing::{post, patch, get, delete};
use tower_http::cors::{Any, CorsLayer};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use axum::http::header;
use once_cell::sync::Lazy;
use tokio::net::TcpListener;


static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);


#[tokio::main]
async fn main() {

    // Connect to database
    DB.connect::<Ws>("0.0.0.0:8000").await.expect("Failed to connect to database");

    // Init tracing
    tracing_subscriber::fmt::init();

    // Routes
    let app = Router::new()
        .route("/auth/signup", post(endpoints::auth::create_user))
        .route("/auth/login", post(endpoints::auth::login))
        .route("/auth", patch(endpoints::auth::update_credentials))
        .route("/album/create", post(endpoints::album::create_album))
        .route("/album/get", get(endpoints::album::get_albums))
        .route("/album/rename", patch(endpoints::album::rename_album))
        .route("/album/delete", delete(endpoints::album::delete_album))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        );

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}