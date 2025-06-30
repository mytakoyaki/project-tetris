use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::achievement::{Achievement, GameStats, AchievementCategory, AchievementCondition};

#[wasm_bindgen]
pub struct AchievementManager {
    achievements: Vec<Achievement>,
    total_points: u32,
    unlocked_count: u32,
}

#[wasm_bindgen]
impl AchievementManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let achievements = Self::create_achievements();
        let unlocked_count = achievements.iter().filter(|a| a.unlocked).count() as u32;
        let total_points = achievements.iter()
            .filter(|a| a.unlocked)
            .map(|a| a.point_reward)
            .sum();

        Self {
            achievements,
            total_points,
            unlocked_count,
        }
    }

    pub fn get_achievements(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.achievements).unwrap()
    }

    pub fn get_total_points(&self) -> u32 {
        self.total_points
    }

    pub fn get_unlocked_count(&self) -> u32 {
        self.unlocked_count
    }

    pub fn check_achievements(&mut self, stats: JsValue) -> JsValue {
        let game_stats: GameStats = serde_wasm_bindgen::from_value(stats).unwrap();
        let mut newly_unlocked = Vec::new();

        for achievement in &mut self.achievements {
            if !achievement.unlocked {
                let progress = Self::calculate_progress(&achievement, &game_stats);
                achievement.progress = progress;

                if progress >= achievement.max_progress {
                    achievement.unlocked = true;
                    achievement.unlocked_at = Some(chrono::Utc::now().to_rfc3339());
                    self.total_points += achievement.point_reward;
                    self.unlocked_count += 1;
                    newly_unlocked.push(achievement.clone());
                }
            }
        }

        serde_wasm_bindgen::to_value(&newly_unlocked).unwrap()
    }

    fn calculate_progress(achievement: &Achievement, stats: &GameStats) -> u32 {
        match achievement.condition.condition_type.as_str() {
            "score" => stats.score.min(achievement.max_progress),
            "lines_cleared" => stats.lines_cleared.min(achievement.max_progress),
            "blocks_placed" => stats.blocks_placed.min(achievement.max_progress),
            "tetris" => stats.tetris_count.min(achievement.max_progress),
            "tspin" => stats.tspin_count.min(achievement.max_progress),
            "max_combo" => stats.max_combo.min(achievement.max_progress),
            "perfect_clear" => stats.perfect_clear_count.min(achievement.max_progress),
            "fever_count" => stats.fever_count.min(achievement.max_progress),
            "exchange_count" => stats.exchange_count.min(achievement.max_progress),
            "hold_count" => stats.hold_count.min(achievement.max_progress),
            "level" => stats.level.min(achievement.max_progress),
            "dan_rank" => stats.dan_rank.min(achievement.max_progress),
            "games_played" => stats.games_played.min(achievement.max_progress),
            _ => 0,
        }
    }

    fn create_achievements() -> Vec<Achievement> {
        vec![
            Achievement {
                id: "first_line".to_string(),
                name: "初回ライン消去".to_string(),
                description: "初めてラインを消去しました".to_string(),
                category: AchievementCategory::Basic,
                icon: "🏁".to_string(),
                point_reward: 10,
                unlocked: false,
                unlocked_at: None,
                progress: 0,
                max_progress: 1,
                condition: AchievementCondition {
                    condition_type: "lines_cleared".to_string(),
                    value: 1,
                    score: None,
                    time: None,
                    max_blocks: None,
                },
                hidden: false,
            },
            Achievement {
                id: "first_game".to_string(),
                name: "はじめての一歩".to_string(),
                description: "初回ゲームプレイ".to_string(),
                category: AchievementCategory::Basic,
                icon: "🎮".to_string(),
                point_reward: 5,
                unlocked: false,
                unlocked_at: None,
                progress: 0,
                max_progress: 1,
                condition: AchievementCondition {
                    condition_type: "games_played".to_string(),
                    value: 1,
                    score: None,
                    time: None,
                    max_blocks: None,
                },
                hidden: false,
            },
            // 他の実績も同様に追加（簡略化のため2つのみ）
        ]
    }
} 