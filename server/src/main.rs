use std::net::SocketAddr;
use std::path::Path;

use axum::http::{HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;

mod api;
mod config;
mod db;
mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub secret: Vec<u8>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let cfg = config::Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    let migrator = sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Failed to load migrations");
    migrator
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        pool,
        secret: cfg.secret,
    };

    let allowed_origins = [
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        "http://localhost:3001".parse::<HeaderValue>().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::COOKIE,
        ])
        .allow_credentials(true);

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(60)
        .finish()
        .unwrap();
    let governor_layer = GovernorLayer {
        config: std::sync::Arc::new(governor_conf),
    };

    let app = api::router()
        .with_state(state)
        .layer(cors)
        .layer(governor_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    tracing::info!("Stackpedia server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
