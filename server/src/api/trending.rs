use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::db::{models::*, queries};
use crate::AppState;

pub async fn get_trending(
    State(state): State<AppState>,
) -> Result<Json<TrendingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let days = 7;

    let stacks = queries::get_trending_stacks(&state.pool, days)
        .await
        .map_err(|e| {
            tracing::error!("get_trending_stacks: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    let hot = queries::get_hot_tools(&state.pool, days)
        .await
        .map_err(|e| {
            tracing::error!("get_hot_tools: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    let regretted = queries::get_most_regretted(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("get_most_regretted: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(TrendingResponse {
        top_stacks: stacks
            .into_iter()
            .map(|r| TrendingStack {
                id: r.id,
                project_name: r.project_name,
                description: r.description,
                category: r.category,
                scale: r.scale,
                recent_votes: r.recent_votes.unwrap_or(0),
                creator_nickname: r.creator_nickname.unwrap_or_default(),
            })
            .collect(),
        hot_tools: hot
            .into_iter()
            .map(|r| TrendingTool {
                name: r.name.unwrap_or_default(),
                category: r.category.unwrap_or_default(),
                count: r.count.unwrap_or(0),
            })
            .collect(),
        most_regretted: regretted
            .into_iter()
            .map(|r| TrendingTool {
                name: r.name.unwrap_or_default(),
                category: r.category.unwrap_or_default(),
                count: r.count.unwrap_or(0),
            })
            .collect(),
    }))
}
