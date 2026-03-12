use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::{models::*, queries};
use crate::middleware::session::Session;
use crate::AppState;

pub async fn add_bookmark(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Verify stack exists
    queries::get_stack_by_id(&state.pool, stack_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "internal error".into() }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "stack not found".into() }),
        ))?;

    queries::add_bookmark(&state.pool, Uuid::new_v4(), session.user_id, stack_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "could not bookmark".into() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_bookmark(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    queries::remove_bookmark(&state.pool, session.user_id, stack_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "could not remove bookmark".into() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_bookmarks(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<Vec<StackSummary>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::get_bookmarked_stacks(&state.pool, session.user_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "internal error".into() }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| StackSummary {
                id: r.id,
                project_name: r.project_name,
                description: r.description,
                category: r.category,
                url: r.url,
                scale: r.scale,
                upvotes: r.upvotes,
                tool_count: r.tool_count.unwrap_or(0),
                comment_count: r.comment_count.unwrap_or(0),
                creator_nickname: r.creator_nickname.unwrap_or_default(),
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect(),
    ))
}

pub async fn check_bookmark(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<Uuid>,
) -> Result<Json<BookmarkCheckResponse>, (StatusCode, Json<ErrorResponse>)> {
    let bookmarked = queries::is_bookmarked(&state.pool, session.user_id, stack_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "internal error".into() }),
            )
        })?;

    Ok(Json(BookmarkCheckResponse { bookmarked }))
}
