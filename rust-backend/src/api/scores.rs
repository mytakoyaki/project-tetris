use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::{Score, CreateScoreRequest, ScoreWithUser};

#[derive(Deserialize)]
pub struct ScoreQuery {
    pub game_mode: Option<String>,
    pub limit: Option<i64>,
}

pub fn scores_router() -> axum::Router<Pool> {
    axum::Router::new()
        .route("/", axum::routing::get(get_scores))
        .route("/", axum::routing::post(create_score))
        .route("/:id", axum::routing::get(get_score))
        .route("/ranking", axum::routing::get(get_ranking))
}

async fn get_scores(
    State(_pool): State<Pool>,
    Query(query): Query<ScoreQuery>,
) -> Result<Json<Vec<Score>>, StatusCode> {
    // TODO: データベースからスコア取得
    let scores = vec![
        Score {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            score: 10000,
            lines_cleared: 50,
            level: 5,
            game_mode: query.game_mode.unwrap_or_else(|| "classic".to_string()),
            created_at: chrono::Utc::now(),
        }
    ];
    
    Ok(Json(scores))
}

async fn create_score(
    State(_pool): State<Pool>,
    Json(payload): Json<CreateScoreRequest>,
) -> Result<Json<Score>, StatusCode> {
    // TODO: データベースにスコア作成
    let score = Score {
        id: Uuid::new_v4(),
        user_id: payload.user_id,
        score: payload.score,
        lines_cleared: payload.lines_cleared,
        level: payload.level,
        game_mode: payload.game_mode,
        created_at: chrono::Utc::now(),
    };
    
    Ok(Json(score))
}

async fn get_score(
    State(_pool): State<Pool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Score>, StatusCode> {
    // TODO: データベースからスコア取得
    let score = Score {
        id,
        user_id: Uuid::new_v4(),
        score: 10000,
        lines_cleared: 50,
        level: 5,
        game_mode: "classic".to_string(),
        created_at: chrono::Utc::now(),
    };
    
    Ok(Json(score))
}

async fn get_ranking(
    State(_pool): State<Pool>,
    Query(query): Query<ScoreQuery>,
) -> Result<Json<Vec<ScoreWithUser>>, StatusCode> {
    // TODO: データベースからランキング取得
    let ranking = vec![
        ScoreWithUser {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            username: "top_player".to_string(),
            score: 50000,
            lines_cleared: 200,
            level: 15,
            game_mode: query.game_mode.unwrap_or_else(|| "classic".to_string()),
            created_at: chrono::Utc::now(),
        }
    ];
    
    Ok(Json(ranking))
} 