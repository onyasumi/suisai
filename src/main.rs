mod controllers;
mod utils;
pub(crate) mod models;

use axum::Router;
use axum::routing::{get, post, patch};
use tower_http::cors::{Any, CorsLayer};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use std::net::SocketAddr;
use axum::http::header;
use tokio::net::TcpListener;


static DB: Surreal<Client> = Surreal::init();


#[tokio::main]
async fn main() {

    // Connect to database
    DB.connect::<Ws>("0.0.0.0:8000").await.expect("Failed to connect to database");

    // Init tracing
    tracing_subscriber::fmt::init();

    // Routes
    let app = Router::new()
        .route("/auth/signup", post(controllers::auth::create_user))
        .route("/auth/login", post(controllers::auth::login))
        .route("/auth", patch(controllers::auth::update_credentials))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        );

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}