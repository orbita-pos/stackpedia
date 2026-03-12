use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::db::{models::*, queries};
use crate::AppState;

fn compute_avg_verdict(verdicts_csv: &str) -> String {
    let scores: Vec<f64> = verdicts_csv
        .split(',')
        .filter_map(|v| match v.trim() {
            "love" => Some(4.0),
            "good" => Some(3.0),
            "meh" => Some(2.0),
            "regret" => Some(1.0),
            _ => None,
        })
        .collect();

    if scores.is_empty() {
        return "unknown".into();
    }

    let avg = scores.iter().sum::<f64>() / scores.len() as f64;
    match avg {
        x if x >= 3.5 => "love".into(),
        x if x >= 2.5 => "good".into(),
        x if x >= 1.5 => "meh".into(),
        _ => "regret".into(),
    }
}

pub async fn list_tools(
    State(state): State<AppState>,
) -> Result<Json<Vec<ToolDirectoryEntry>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::get_tools_directory(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("get_tools_directory: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| ToolDirectoryEntry {
                name: r.name.unwrap_or_default(),
                category: r.category.unwrap_or_default(),
                stack_count: r.stack_count.unwrap_or(0),
                avg_verdict: compute_avg_verdict(&r.verdicts.unwrap_or_default()),
            })
            .collect(),
    ))
}

pub async fn get_tool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<ToolDetailEntry>>, (StatusCode, Json<ErrorResponse>)> {
    let tool_name = name.to_lowercase();
    let rows = queries::get_stacks_for_tool(&state.pool, &tool_name)
        .await
        .map_err(|e| {
            tracing::error!("get_stacks_for_tool: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| ToolDetailEntry {
                stack_id: r.stack_id.unwrap_or_default(),
                project_name: r.project_name.unwrap_or_default(),
                why: r.why,
                verdict: r.verdict,
                cost: r.cost,
            })
            .collect(),
    ))
}

pub async fn get_tool_pairs(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<ToolPairingEntry>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::get_tool_pairs(&state.pool, &name)
        .await
        .map_err(|e| {
            tracing::error!("get_tool_pairs: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| ToolPairingEntry {
                name: r.name.unwrap_or_default(),
                category: r.category.unwrap_or_default(),
                pair_count: r.pair_count.unwrap_or(0),
            })
            .collect(),
    ))
}

pub async fn get_tool_alternatives(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<ToolAlternative>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::get_tool_alternatives(&state.pool, &name)
        .await
        .map_err(|e| {
            tracing::error!("get_tool_alternatives: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| ToolAlternative {
                name: r.name.unwrap_or_default(),
                category: r.category.unwrap_or_default(),
                times_chosen: r.times_chosen.unwrap_or(0),
                avg_verdict: r.avg_verdict.unwrap_or_default(),
            })
            .collect(),
    ))
}

pub async fn stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let row = queries::get_stats(&state.pool).await.map_err(|e| {
        tracing::error!("get_stats: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal error".into(),
            }),
        )
    })?;

    Ok(Json(StatsResponse {
        total_stacks: row.total_stacks.unwrap_or(0),
        total_tools: row.total_tools.unwrap_or(0),
        total_users: row.total_users.unwrap_or(0),
    }))
}
