use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::db::Pool;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};

pub fn users_router() -> axum::Router<Pool> {
    axum::Router::new()
        .route("/", axum::routing::get(get_users))
        .route("/", axum::routing::post(create_user))
        .route("/:id", axum::routing::get(get_user))
        .route("/:id", axum::routing::put(update_user))
        .route("/:id", axum::routing::delete(delete_user))
}

async fn get_users(
    State(_pool): State<Pool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    // TODO: データベースからユーザー取得
    let users = vec![
        User {
            id: Uuid::new_v4(),
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];
    
    Ok(Json(users))
}

async fn create_user(
    State(_pool): State<Pool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // TODO: データベースにユーザー作成
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(user))
}

async fn get_user(
    State(_pool): State<Pool>,
    Path(id): Path<Uuid>,
    ) -> Result<Json<User>, StatusCode> {
    // TODO: データベースからユーザー取得
    let user = User {
        id,
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(user))
}

async fn update_user(
    State(_pool): State<Pool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // TODO: データベースでユーザー更新
    let user = User {
        id,
        username: payload.username.unwrap_or_else(|| "test_user".to_string()),
        email: payload.email.unwrap_or_else(|| "test@example.com".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(user))
}

async fn delete_user(
    State(_pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: データベースからユーザー削除
    Ok(StatusCode::NO_CONTENT)
} 