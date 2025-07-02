use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAchievementRequest {
    pub user_id: Uuid,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub achievement_type: String,
    pub title: String,
    pub description: String,
    pub unlocked_at: DateTime<Utc>,
} 