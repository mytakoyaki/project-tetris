use uuid::Uuid;
use crate::models::{CreateScoreRequest, CreateAchievementRequest};

pub struct GameService;

impl GameService {
    pub fn calculate_score(lines_cleared: i32, level: i32) -> i32 {
        // 基本的なスコア計算ロジック
        let base_score = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        
        base_score * level
    }

    pub fn check_achievements(score: i32, lines_cleared: i32, level: i32) -> Vec<CreateAchievementRequest> {
        let mut achievements = Vec::new();
        
        // スコア関連の実績
        if score >= 10000 {
            achievements.push(CreateAchievementRequest {
                user_id: Uuid::nil(), // 実際のユーザーIDに置き換える
                achievement_type: "score".to_string(),
                title: "High Scorer".to_string(),
                description: "Reach 10,000 points".to_string(),
            });
        }
        
        // ライン消去関連の実績
        if lines_cleared >= 100 {
            achievements.push(CreateAchievementRequest {
                user_id: Uuid::nil(), // 実際のユーザーIDに置き換える
                achievement_type: "lines".to_string(),
                title: "Line Master".to_string(),
                description: "Clear 100 lines".to_string(),
            });
        }
        
        // レベル関連の実績
        if level >= 10 {
            achievements.push(CreateAchievementRequest {
                user_id: Uuid::nil(), // 実際のユーザーIDに置き換える
                achievement_type: "level".to_string(),
                title: "Level Up".to_string(),
                description: "Reach level 10".to_string(),
            });
        }
        
        achievements
    }
} 