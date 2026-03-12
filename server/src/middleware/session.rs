use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use crate::AppState;

type HmacSha256 = Hmac<Sha256>;

pub struct Session {
    pub user_id: Uuid,
}

pub fn sign_token(user_id: Uuid, secret: &[u8]) -> String {
    let payload = user_id.to_string();
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key length is valid");
    mac.update(payload.as_bytes());
    let sig = hex::encode(mac.finalize().into_bytes());
    format!("{}.{}", payload, sig)
}

pub fn verify_token(token: &str, secret: &[u8]) -> Option<Uuid> {
    let (payload, sig) = token.split_once('.')?;
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key length is valid");
    mac.update(payload.as_bytes());
    let expected_sig = hex::decode(sig).ok()?;
    mac.verify_slice(&expected_sig).ok()?;
    payload.parse().ok()
}

fn extract_user_id(parts: &Parts, secret: &[u8]) -> Option<Uuid> {
    let cookie_header = parts.headers.get("cookie")?.to_str().ok()?;
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("stackpedia_session=") {
            return verify_token(value, secret);
        }
    }
    None
}

impl FromRequestParts<AppState> for Session {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match extract_user_id(parts, &state.secret) {
            Some(user_id) => Ok(Session { user_id }),
            None => Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "not authenticated"})),
            )),
        }
    }
}

