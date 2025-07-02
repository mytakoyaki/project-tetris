use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::db::Pool;
use crate::services::AuthService;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyTokenResponse {
    pub valid: bool,
    pub user_id: Option<String>,
}

pub fn auth_router() -> axum::Router<Pool> {
    axum::Router::new()
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::post(verify_token))
        .route("/logout", axum::routing::post(logout))
}

async fn login(
    State(_pool): State<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // TODO: 実際の認証ロジック（Clerk連携）
    // 現在はダミー実装
    if payload.email == "test@example.com" && payload.password == "password" {
        let token = AuthService::create_token("test-user-id")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(LoginResponse {
            token,
            user_id: "test-user-id".to_string(),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn verify_token(
    State(_pool): State<Pool>,
    Json(payload): Json<VerifyTokenRequest>,
) -> Result<Json<VerifyTokenResponse>, StatusCode> {
    match AuthService::verify_token(&payload.token) {
        Ok(claims) => Ok(Json(VerifyTokenResponse {
            valid: true,
            user_id: Some(claims.sub),
        })),
        Err(_) => Ok(Json(VerifyTokenResponse {
            valid: false,
            user_id: None,
        })),
    }
}

async fn logout(
    State(_pool): State<Pool>,
) -> Result<StatusCode, StatusCode> {
    // TODO: トークンの無効化（Redis等でブラックリスト管理）
    Ok(StatusCode::OK)
} 