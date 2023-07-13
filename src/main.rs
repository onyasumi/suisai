mod controllers;
mod utils;
mod models;

use axum::Router;
use axum::routing::{get, post, patch};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use std::net::SocketAddr;

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
        .route("/auth", patch(controllers::auth::update_credentials));

    // Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}