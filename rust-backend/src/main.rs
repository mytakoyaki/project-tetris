mod api;
mod config;
mod db;
mod models;
mod services;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::establish_connection;

#[tokio::main]
async fn main() {
    // 設定読み込み
    let config = AppConfig::from_env().expect("Failed to load configuration");
    
    // ログ設定
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // データベース接続プール作成
    let pool = establish_connection();

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ルーター作成
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/api/users", api::users_router())
        .nest("/api/scores", api::scores_router())
        .nest("/api/achievements", api::achievements_router())
        .nest("/api/auth", api::auth_router())
        .layer(cors)
        .with_state(pool);

    // サーバー起動
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "ClaudeTetris Rust Backend API"
}

async fn health() -> &'static str {
    "OK"
}
