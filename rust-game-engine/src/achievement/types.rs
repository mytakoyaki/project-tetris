use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub icon: String,
    pub point_reward: u32,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
    pub progress: u32,
    pub max_progress: u32,
    pub condition: AchievementCondition,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Basic,
    Score,
    Technical,
    Challenge,
    Special,
    Rank,
    Progress,
    Fun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementCondition {
    pub condition_type: String,
    pub value: u32,
    pub score: Option<u32>,
    pub time: Option<u32>,
    pub max_blocks: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub score: u32,
    pub lines_cleared: u32,
    pub blocks_placed: u32,
    pub tetris_count: u32,
    pub tspin_count: u32,
    pub max_combo: u32,
    pub perfect_clear_count: u32,
    pub fever_count: u32,
    pub exchange_count: u32,
    pub hold_count: u32,
    pub level: u32,
    pub dan_rank: u32,
    pub play_time: u32,
    pub games_played: u32,
} 