#[cfg(feature = "validator")]
use std::borrow::Cow;
#[cfg(feature = "validator")]
use std::sync::LazyLock;

#[cfg(feature = "validator")]
use validator::ValidationError;

#[cfg(feature = "validator")]
pub static ERROR_ALREADY_EXISTS: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("already-exists").with_message(Cow::Borrowed("Already exists")));
#[cfg(feature = "validator")]
pub static ERROR_IS_INVALID: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("invalid").with_message(Cow::Borrowed("Is invalid")));
