use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::{Achievement, CreateAchievementRequest, AchievementWithUser};

#[derive(Deserialize)]
pub struct AchievementQuery {
    pub user_id: Option<Uuid>,
    pub achievement_type: Option<String>,
}

pub fn achievements_router() -> axum::Router<Pool> {
    axum::Router::new()
        .route("/", axum::routing::get(get_achievements))
        .route("/", axum::routing::post(create_achievement))
        .route("/:id", axum::routing::get(get_achievement))
        .route("/user/:user_id", axum::routing::get(get_user_achievements))
}

async fn get_achievements(
    State(_pool): State<Pool>,
    Query(query): Query<AchievementQuery>,
) -> Result<Json<Vec<Achievement>>, StatusCode> {
    // TODO: データベースから実績取得
    let achievements = vec![
        Achievement {
            id: Uuid::new_v4(),
            user_id: query.user_id.unwrap_or_else(Uuid::new_v4),
            achievement_type: query.achievement_type.unwrap_or_else(|| "score".to_string()),
            title: "High Scorer".to_string(),
            description: "Reach 10,000 points".to_string(),
            unlocked_at: chrono::Utc::now(),
        }
    ];
    
    Ok(Json(achievements))
}

async fn create_achievement(
    State(_pool): State<Pool>,
    Json(payload): Json<CreateAchievementRequest>,
) -> Result<Json<Achievement>, StatusCode> {
    // TODO: データベースに実績作成
    let achievement = Achievement {
        id: Uuid::new_v4(),
        user_id: payload.user_id,
        achievement_type: payload.achievement_type,
        title: payload.title,
        description: payload.description,
        unlocked_at: chrono::Utc::now(),
    };
    
    Ok(Json(achievement))
}

async fn get_achievement(
    State(_pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Achievement>, StatusCode> {
    // TODO: データベースから実績取得
    let achievement = Achievement {
        id,
        user_id: Uuid::new_v4(),
        achievement_type: "score".to_string(),
        title: "High Scorer".to_string(),
        description: "Reach 10,000 points".to_string(),
        unlocked_at: chrono::Utc::now(),
    };
    
    Ok(Json(achievement))
}

async fn get_user_achievements(
    State(_pool): State<Pool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<AchievementWithUser>>, StatusCode> {
    // TODO: データベースからユーザーの実績取得
    let achievements = vec![
        AchievementWithUser {
            id: Uuid::new_v4(),
            user_id,
            username: "test_user".to_string(),
            achievement_type: "score".to_string(),
            title: "High Scorer".to_string(),
            description: "Reach 10,000 points".to_string(),
            unlocked_at: chrono::Utc::now(),
        }
    ];
    
    Ok(Json(achievements))
} 