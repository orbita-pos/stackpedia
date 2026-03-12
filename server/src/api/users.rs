use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::db::{models::*, queries};
use crate::middleware::session::Session;
use crate::AppState;

pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(nickname): Path<String>,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = queries::get_user_by_nickname(&state.pool, &nickname)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "database error".into(),
                }),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "user not found".into(),
            }),
        )
    })?;

    let stacks = queries::get_stacks_by_creator(&state.pool, user.id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "database error".into(),
                }),
            )
        })?;

    let stack_count = stacks.len() as i64;
    let stacks: Vec<StackSummary> = stacks
        .into_iter()
        .map(|s| StackSummary {
            id: s.id,
            project_name: s.project_name,
            description: s.description,
            category: s.category,
            url: s.url,
            scale: s.scale,
            upvotes: s.upvotes,
            tool_count: s.tool_count.unwrap_or(0),
            comment_count: s.comment_count.unwrap_or(0),
            creator_nickname: s.creator_nickname.unwrap_or_default(),
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();

    Ok(Json(UserProfileResponse {
        nickname: user.nickname,
        created_at: user.created_at,
        stack_count,
        sponsor_url: user.sponsor_url,
        stacks,
    }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    session: Session,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Validate sponsor_url if provided
    if let Some(ref url) = body.sponsor_url {
        if url.len() > 200 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "sponsor_url must be max 200 chars".into(),
                }),
            ));
        }
        if !url.is_empty() && !url.starts_with("https://") {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "sponsor_url must start with https://".into(),
                }),
            ));
        }
    }

    let sponsor_url = body
        .sponsor_url
        .as_deref()
        .filter(|s| !s.is_empty());

    queries::update_sponsor_url(&state.pool, session.user_id, sponsor_url)
        .await
        .map_err(|e| {
            tracing::error!("update_sponsor_url: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "could not update profile".into(),
                }),
            )
        })?;

    Ok(Json(serde_json::json!({ "ok": true })))
}
