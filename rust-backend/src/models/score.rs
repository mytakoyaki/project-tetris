use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub id: Uuid,
    pub user_id: Uuid,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScoreRequest {
    pub user_id: Uuid,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub score: i32,
    pub lines_cleared: i32,
    pub level: i32,
    pub game_mode: String,
    pub created_at: DateTime<Utc>,
} 