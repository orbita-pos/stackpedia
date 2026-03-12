use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;
use uuid::Uuid;

use crate::db::{models::*, queries};
use crate::middleware::session::{sign_token, Session};
use crate::AppState;

fn generate_recovery_code() -> (String, String) {
    let mut rng = rand::thread_rng();
    let part1: u16 = rng.gen_range(0..=9999);
    let part2: u16 = rng.gen_range(0..=9999);
    let code = format!("STACK-{:04}-{:04}", part1, part2);
    let prefix = format!("STACK-{:04}", part1);
    (code, prefix)
}

fn hash_recovery_code(code: &str) -> Result<String, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(code.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_recovery_code(code: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .ok()
        .and_then(|parsed| Argon2::default().verify_password(code.as_bytes(), &parsed).ok())
        .is_some()
}

fn set_session_cookie(token: &str, is_production: bool) -> String {
    if is_production {
        format!(
            "stackpedia_session={}; Path=/; HttpOnly; SameSite=Strict; Secure; Max-Age=2592000",
            token
        )
    } else {
        format!(
            "stackpedia_session={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=2592000",
            token
        )
    }
}

type ApiError = (StatusCode, Json<ErrorResponse>);

fn err(status: StatusCode, msg: &str) -> ApiError {
    (status, Json(ErrorResponse { error: msg.into() }))
}

pub async fn join(
    State(state): State<AppState>,
    Json(body): Json<JoinRequest>,
) -> Result<Response, ApiError> {
    let nickname = body.nickname.trim().to_string();
    if nickname.is_empty() || nickname.len() > 30 {
        return Err(err(StatusCode::BAD_REQUEST, "nickname must be 1-30 chars"));
    }
    if !nickname.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(err(StatusCode::BAD_REQUEST, "nickname may only contain a-z, 0-9, _ and -"));
    }

    let (recovery_code, recovery_prefix) = generate_recovery_code();
    let hash = hash_recovery_code(&recovery_code)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "internal error"))?;

    // Check nickname availability
    let existing = queries::get_user_by_nickname(&state.pool, &nickname)
        .await
        .map_err(|e| {
            tracing::error!("get_user_by_nickname: {}", e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
        })?;
    if existing.is_some() {
        return Err(err(StatusCode::CONFLICT, "nickname already taken"));
    }

    let user_id = Uuid::new_v4();
    queries::insert_user(&state.pool, user_id, &nickname, &hash, Some(&recovery_prefix))
        .await
        .map_err(|e| {
            tracing::error!("insert_user: {}", e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "could not create user")
        })?;

    let token = sign_token(user_id, &state.secret);

    let mut response = (
        StatusCode::CREATED,
        Json(JoinResponse {
            user_id,
            nickname,
            recovery_code,
        }),
    )
        .into_response();

    response.headers_mut().insert(
        SET_COOKIE,
        set_session_cookie(&token, state.is_production).parse().unwrap(),
    );

    Ok(response)
}

pub async fn recover(
    State(state): State<AppState>,
    Json(body): Json<RecoverRequest>,
) -> Result<Response, ApiError> {
    let code = body.recovery_code.trim().to_string();
    if code.is_empty() {
        return Err(err(StatusCode::BAD_REQUEST, "recovery_code is required"));
    }

    // Validate format: STACK-DDDD-DDDD
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() != 3
        || parts[0] != "STACK"
        || parts[1].len() != 4
        || parts[2].len() != 4
        || !parts[1].chars().all(|c| c.is_ascii_digit())
        || !parts[2].chars().all(|c| c.is_ascii_digit())
    {
        return Err(err(StatusCode::BAD_REQUEST, "invalid recovery code format"));
    }

    // Extract prefix (e.g. "STACK-1234" from "STACK-1234-5678") to narrow search
    let prefix = format!("{}-{}", parts[0], parts[1]);

    let users = if prefix.starts_with("STACK-") && prefix.len() >= 6 {
        let mut users = queries::get_users_by_recovery_prefix(&state.pool, &prefix)
            .await
            .map_err(|e| {
                tracing::error!("get_users_by_recovery_prefix: {}", e);
                err(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            })?;
        // Fallback to full scan for pre-migration users with NULL prefix
        if users.is_empty() {
            users = queries::get_all_users(&state.pool).await.map_err(|e| {
                tracing::error!("get_all_users: {}", e);
                err(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            })?;
        }
        users
    } else {
        queries::get_all_users(&state.pool).await.map_err(|e| {
            tracing::error!("get_all_users: {}", e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
        })?
    };

    for user in &users {
        if verify_recovery_code(&code, &user.recovery_code_hash) {
            let token = sign_token(user.id, &state.secret);

            let mut response = Json(UserResponse {
                user_id: user.id,
                nickname: user.nickname.clone(),
            })
            .into_response();

            response.headers_mut().insert(
                SET_COOKIE,
                set_session_cookie(&token, state.is_production).parse().unwrap(),
            );

            return Ok(response);
        }
    }

    Err(err(StatusCode::UNAUTHORIZED, "invalid recovery code"))
}

pub async fn me(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<UserResponse>, ApiError> {
    let user = queries::get_user_by_id(&state.pool, session.user_id)
        .await
        .map_err(|e| {
            tracing::error!("get_user_by_id: {}", e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
        })?
        .ok_or(err(StatusCode::UNAUTHORIZED, "user not found"))?;

    Ok(Json(UserResponse {
        user_id: user.id,
        nickname: user.nickname,
    }))
}
