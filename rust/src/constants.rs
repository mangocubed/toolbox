#[cfg(feature = "axum")]
use axum::Json;
#[cfg(feature = "axum")]
use axum::http::StatusCode;
#[cfg(feature = "validator")]
use std::borrow::Cow;

use std::sync::LazyLock;

#[cfg(feature = "validator")]
use validator::ValidationError;

#[cfg(feature = "validator")]
pub static ERROR_ALREADY_EXISTS: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("already-exists").with_message(Cow::Borrowed("Already exists")));
#[cfg(feature = "validator")]
pub static ERROR_IS_INVALID: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("invalid").with_message(Cow::Borrowed("Is invalid")));

#[cfg(feature = "axum")]
pub static RESPONSE_ERROR_BAD_REQUEST: LazyLock<(StatusCode, Json<serde_json::Value>)> = LazyLock::new(|| {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"message": "Bad Request"})),
    )
});
#[cfg(feature = "axum")]
pub static RESPONSE_ERROR_FORBIDDEN: LazyLock<(StatusCode, Json<serde_json::Value>)> =
    LazyLock::new(|| (StatusCode::FORBIDDEN, Json(serde_json::json!({"message": "Forbidden"}))));
#[cfg(feature = "axum")]
pub static RESPONSE_ERROR_INTERNAL_SERVER_ERROR: LazyLock<(StatusCode, Json<serde_json::Value>)> =
    LazyLock::new(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Internal Server Error"})),
        )
    });
#[cfg(feature = "axum")]
pub static RESPONSE_ERROR_NOT_FOUND: LazyLock<(StatusCode, Json<serde_json::Value>)> =
    LazyLock::new(|| (StatusCode::NOT_FOUND, Json(serde_json::json!({"message": "Not Found"}))));
#[cfg(feature = "axum")]
pub static RESPONSE_ERROR_UNAUTHORIZED: LazyLock<(StatusCode, Json<serde_json::Value>)> = LazyLock::new(|| {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"message": "Unauthorized"})),
    )
});
