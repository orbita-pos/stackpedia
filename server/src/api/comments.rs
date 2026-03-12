use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::{models::*, queries};
use crate::middleware::session::Session;
use crate::AppState;

pub async fn create_comment(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<Uuid>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let content = body.content.trim().to_string();
    if content.is_empty() || content.len() > 500 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "content must be 1-500 chars".into(),
            }),
        ));
    }

    // Verify stack exists
    queries::get_stack_by_id(&state.pool, stack_id)
        .await
        .map_err(|e| {
            tracing::error!("get_stack_by_id: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "stack not found".into(),
            }),
        ))?;

    let comment_id = Uuid::new_v4();
    let comment = queries::insert_comment(&state.pool, comment_id, stack_id, session.user_id, &content)
        .await
        .map_err(|e| {
            tracing::error!("insert_comment: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "could not create comment".into(),
                }),
            )
        })?;

    let nickname = queries::get_creator_nickname(&state.pool, session.user_id)
        .await
        .unwrap_or_else(|_| "unknown".into());

    Ok((
        StatusCode::CREATED,
        Json(CommentResponse {
            id: comment.id,
            content: comment.content,
            creator_nickname: nickname,
            created_at: comment.created_at,
        }),
    ))
}

pub async fn list_comments(
    State(state): State<AppState>,
    Path(stack_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::get_comments_for_stack(&state.pool, stack_id)
        .await
        .map_err(|e| {
            tracing::error!("get_comments_for_stack: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|c| CommentResponse {
                id: c.id,
                content: c.content,
                creator_nickname: c.creator_nickname.unwrap_or_default(),
                created_at: c.created_at,
            })
            .collect(),
    ))
}
