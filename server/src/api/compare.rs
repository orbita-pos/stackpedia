use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::db::{models::*, queries};
use crate::AppState;

pub async fn compare_tools(
    State(state): State<AppState>,
    Query(query): Query<CompareToolsQuery>,
) -> Result<Json<CompareResponse>, (StatusCode, Json<ErrorResponse>)> {
    let tool_names: Vec<&str> = query.tools.split(',').map(|s| s.trim()).collect();

    if tool_names.len() != 2 || tool_names.iter().any(|n| n.is_empty()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "provide exactly 2 comma-separated tool names".into(),
            }),
        ));
    }

    let mut entries = Vec::new();
    for &name in &tool_names {
        let row = queries::get_tool_comparison(&state.pool, name)
            .await
            .map_err(|e| {
                tracing::error!("get_tool_comparison: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "internal error".into(),
                    }),
                )
            })?;

        let whys = queries::get_sample_whys(&state.pool, name, 3)
            .await
            .unwrap_or_default();
        let costs = queries::get_common_costs(&state.pool, name, 5)
            .await
            .unwrap_or_default();

        entries.push(ToolComparisonEntry {
            name: name.to_string(),
            category: row.category.unwrap_or_default(),
            stack_count: row.stack_count.unwrap_or(0),
            verdict_distribution: VerdictDistribution {
                love: row.love.unwrap_or(0),
                good: row.good.unwrap_or(0),
                meh: row.meh.unwrap_or(0),
                regret: row.regret.unwrap_or(0),
            },
            sample_whys: whys,
            common_costs: costs,
        });
    }

    let shared = queries::get_shared_stacks_count(&state.pool, tool_names[0], tool_names[1])
        .await
        .unwrap_or(0);

    Ok(Json(CompareResponse {
        tools: entries,
        shared_stacks: shared,
    }))
}
