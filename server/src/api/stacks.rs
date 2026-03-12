use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::{models::*, queries};
use crate::middleware::session::Session;
use crate::AppState;

pub async fn create_stack(
    State(state): State<AppState>,
    session: Session,
    Json(body): Json<CreateStackRequest>,
) -> Result<(StatusCode, Json<StackDetail>), (StatusCode, Json<ErrorResponse>)> {
    body.validate().map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e }))
    })?;

    let stack_id = Uuid::new_v4();
    let stack = queries::insert_stack(
        &state.pool,
        stack_id,
        session.user_id,
        &body.project_name,
        &body.description,
        &body.category,
        body.url.as_deref(),
        &body.scale,
        body.lessons.as_deref(),
        body.forked_from,
    )
    .await
    .map_err(|e| {
        tracing::error!("insert_stack: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "could not create stack".into(),
            }),
        )
    })?;

    let mut tools_resp = Vec::with_capacity(body.tools.len());
    for tool in &body.tools {
        let tool_id = Uuid::new_v4();
        let row = queries::insert_stack_tool(
            &state.pool,
            tool_id,
            stack_id,
            &tool.name.to_lowercase(),
            &tool.category,
            &tool.why,
            tool.cost.as_deref(),
            &tool.verdict,
        )
        .await
        .map_err(|e| {
            tracing::error!("insert_stack_tool: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "could not create tool".into(),
                }),
            )
        })?;
        tools_resp.push(ToolResponse {
            id: row.id,
            name: row.name,
            category: row.category,
            why: row.why,
            cost: row.cost,
            verdict: row.verdict,
        });
    }

    let nickname = queries::get_creator_nickname(&state.pool, session.user_id)
        .await
        .unwrap_or_else(|_| "unknown".into());

    Ok((
        StatusCode::CREATED,
        Json(StackDetail {
            id: stack.id,
            project_name: stack.project_name,
            description: stack.description,
            category: stack.category,
            url: stack.url,
            scale: stack.scale,
            lessons: stack.lessons,
            upvotes: stack.upvotes,
            creator_id: stack.creator_id,
            creator_nickname: nickname,
            created_at: stack.created_at,
            updated_at: stack.updated_at,
            tools: tools_resp,
            comments: vec![],
            history: vec![],
            forked_from: stack.forked_from,
        }),
    ))
}

pub async fn list_stacks(
    State(state): State<AppState>,
    Query(query): Query<ListStacksQuery>,
) -> Result<Json<Vec<StackSummary>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = queries::list_stacks(&state.pool, &query).await.map_err(|e| {
        tracing::error!("list_stacks: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "could not list stacks".into(),
            }),
        )
    })?;

    let stacks = rows
        .into_iter()
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
        .collect();

    Ok(Json(stacks))
}

pub async fn get_stack(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StackDetail>, (StatusCode, Json<ErrorResponse>)> {
    let id: Uuid = id.parse().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "stack not found".into(),
            }),
        )
    })?;
    let stack = queries::get_stack_by_id(&state.pool, id)
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

    let tools = queries::get_stack_tools(&state.pool, id)
        .await
        .unwrap_or_default();

    let comment_rows = queries::get_comments_for_stack(&state.pool, id)
        .await
        .unwrap_or_default();

    let nickname = queries::get_creator_nickname(&state.pool, stack.creator_id)
        .await
        .unwrap_or_else(|_| "unknown".into());

    let history_rows = queries::get_stack_history(&state.pool, id)
        .await
        .unwrap_or_default();

    Ok(Json(StackDetail {
        id: stack.id,
        project_name: stack.project_name,
        description: stack.description,
        category: stack.category,
        url: stack.url,
        scale: stack.scale,
        lessons: stack.lessons,
        upvotes: stack.upvotes,
        creator_id: stack.creator_id,
        creator_nickname: nickname,
        created_at: stack.created_at,
        updated_at: stack.updated_at,
        tools: tools
            .into_iter()
            .map(|t| ToolResponse {
                id: t.id,
                name: t.name,
                category: t.category,
                why: t.why,
                cost: t.cost,
                verdict: t.verdict,
            })
            .collect(),
        comments: comment_rows
            .into_iter()
            .map(|c| CommentResponse {
                id: c.id,
                content: c.content,
                creator_nickname: c.creator_nickname.unwrap_or_default(),
                created_at: c.created_at,
            })
            .collect(),
        history: history_rows
            .into_iter()
            .map(|h| StackHistoryEntry {
                id: h.id,
                change_type: h.change_type,
                detail: h.detail,
                created_at: h.created_at,
            })
            .collect(),
        forked_from: stack.forked_from,
    }))
}

pub async fn update_stack(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(body): Json<UpdateStackRequest>,
) -> Result<Json<StackDetail>, (StatusCode, Json<ErrorResponse>)> {
    let id: Uuid = id.parse().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "stack not found".into() }),
        )
    })?;
    let stack = queries::get_stack_by_id(&state.pool, id)
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

    if stack.creator_id != session.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "you can only edit your own stacks".into(),
            }),
        ));
    }

    // Track what changed
    let mut changes = Vec::new();
    if body.project_name.is_some() {
        changes.push("project_name");
    }
    if body.description.is_some() {
        changes.push("description");
    }
    if body.category.is_some() {
        changes.push("category");
    }
    if body.scale.is_some() {
        changes.push("scale");
    }
    if body.lessons.is_some() {
        changes.push("lessons");
    }
    if body.tools.is_some() {
        changes.push("tools");
    }

    // Update stack fields
    let updated = queries::update_stack(
        &state.pool,
        id,
        body.project_name.as_deref(),
        body.description.as_deref(),
        body.category.as_deref(),
        body.url.as_deref(),
        body.scale.as_deref(),
        body.lessons.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("update_stack: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "could not update stack".into(),
            }),
        )
    })?;

    // Replace tools if provided
    if let Some(ref new_tools) = body.tools {
        queries::delete_stack_tools(&state.pool, id)
            .await
            .map_err(|e| {
                tracing::error!("delete_stack_tools: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "could not update tools".into(),
                    }),
                )
            })?;

        for tool in new_tools {
            queries::insert_stack_tool(
                &state.pool,
                Uuid::new_v4(),
                id,
                &tool.name.to_lowercase(),
                &tool.category,
                &tool.why,
                tool.cost.as_deref(),
                &tool.verdict,
            )
            .await
            .map_err(|e| {
                tracing::error!("insert_stack_tool: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "could not insert tool".into(),
                    }),
                )
            })?;
        }
    }

    // Record history
    if !changes.is_empty() {
        let detail = format!("updated: {}", changes.join(", "));
        queries::insert_stack_history(
            &state.pool,
            Uuid::new_v4(),
            id,
            session.user_id,
            "update",
            Some(&detail),
        )
        .await
        .ok(); // non-critical
    }

    // Return full updated stack
    let tools = queries::get_stack_tools(&state.pool, id)
        .await
        .unwrap_or_default();
    let comment_rows = queries::get_comments_for_stack(&state.pool, id)
        .await
        .unwrap_or_default();
    let nickname = queries::get_creator_nickname(&state.pool, session.user_id)
        .await
        .unwrap_or_else(|_| "unknown".into());
    let history_rows = queries::get_stack_history(&state.pool, id)
        .await
        .unwrap_or_default();

    Ok(Json(StackDetail {
        id: updated.id,
        project_name: updated.project_name,
        description: updated.description,
        category: updated.category,
        url: updated.url,
        scale: updated.scale,
        lessons: updated.lessons,
        upvotes: updated.upvotes,
        creator_id: updated.creator_id,
        creator_nickname: nickname,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
        tools: tools
            .into_iter()
            .map(|t| ToolResponse {
                id: t.id,
                name: t.name,
                category: t.category,
                why: t.why,
                cost: t.cost,
                verdict: t.verdict,
            })
            .collect(),
        comments: comment_rows
            .into_iter()
            .map(|c| CommentResponse {
                id: c.id,
                content: c.content,
                creator_nickname: c.creator_nickname.unwrap_or_default(),
                created_at: c.created_at,
            })
            .collect(),
        history: history_rows
            .into_iter()
            .map(|h| StackHistoryEntry {
                id: h.id,
                change_type: h.change_type,
                detail: h.detail,
                created_at: h.created_at,
            })
            .collect(),
        forked_from: updated.forked_from,
    }))
}

pub async fn delete_stack(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let id: Uuid = id.parse().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "stack not found".into() }),
        )
    })?;
    let stack = queries::get_stack_by_id(&state.pool, id)
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

    if stack.creator_id != session.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "you can only delete your own stacks".into(),
            }),
        ));
    }

    queries::delete_stack(&state.pool, id).await.map_err(|e| {
        tracing::error!("delete_stack: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "could not delete stack".into(),
            }),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn vote(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<String>,
    Json(body): Json<VoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let stack_id: Uuid = stack_id.parse().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "stack not found".into() }),
        )
    })?;
    let direction: i16 = match body.direction.as_str() {
        "up" => 1,
        "down" => -1,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "direction must be 'up' or 'down'".into(),
                }),
            ))
        }
    };

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

    let existing = queries::get_vote(&state.pool, session.user_id, stack_id)
        .await
        .map_err(|e| {
            tracing::error!("get_vote: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    let action;
    match existing {
        Some(vote) if vote.direction == direction => {
            // Same direction → remove vote
            queries::delete_vote(&state.pool, vote.id)
                .await
                .map_err(|e| {
                    tracing::error!("delete_vote: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "internal error".into(),
                        }),
                    )
                })?;
            action = "removed";
        }
        Some(vote) => {
            // Opposite direction → change vote
            queries::update_vote(&state.pool, vote.id, direction)
                .await
                .map_err(|e| {
                    tracing::error!("update_vote: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "internal error".into(),
                        }),
                    )
                })?;
            action = "changed";
        }
        None => {
            // No existing vote → create
            queries::insert_vote(&state.pool, Uuid::new_v4(), session.user_id, stack_id, direction)
                .await
                .map_err(|e| {
                    tracing::error!("insert_vote: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "internal error".into(),
                        }),
                    )
                })?;
            action = "voted";
        }
    }

    // Recalculate upvotes
    queries::recalc_upvotes(&state.pool, stack_id)
        .await
        .map_err(|e| {
            tracing::error!("recalc_upvotes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".into(),
                }),
            )
        })?;

    // Fetch updated count
    let stack = queries::get_stack_by_id(&state.pool, stack_id)
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
        .unwrap();

    Ok(Json(serde_json::json!({
        "action": action,
        "upvotes": stack.upvotes
    })))
}

pub async fn check_vote(
    State(state): State<AppState>,
    session: Session,
    Path(stack_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let stack_id: Uuid = stack_id.parse().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "stack not found".into() }),
        )
    })?;

    let existing = queries::get_vote(&state.pool, session.user_id, stack_id)
        .await
        .map_err(|e| {
            tracing::error!("get_vote: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "internal error".into() }),
            )
        })?;

    let direction = match existing {
        Some(v) if v.direction == 1 => "up",
        Some(v) if v.direction == -1 => "down",
        _ => "none",
    };

    Ok(Json(serde_json::json!({ "direction": direction })))
}
