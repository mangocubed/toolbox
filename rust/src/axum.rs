use axum::response::Result;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;

use crate::constants::*;

pub type AuthorizationBearer = TypedHeader<Authorization<Bearer>>;

pub trait OrHttpError<T> {
    #[allow(clippy::result_large_err)]
    fn or_bad_request(self) -> Result<T>;

    #[allow(clippy::result_large_err, dead_code)]
    fn or_forbidden(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_internal_server_error(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_not_found(self) -> Result<T>;

    #[allow(clippy::result_large_err)]
    fn or_unauthorized(self) -> Result<T>;
}

impl<T> OrHttpError<T> for Option<T> {
    fn or_bad_request(self) -> Result<T> {
        self.ok_or_else(|| RESPONSE_ERROR_BAD_REQUEST.clone().into())
    }

    fn or_forbidden(self) -> Result<T> {
        self.ok_or_else(|| RESPONSE_ERROR_FORBIDDEN.clone().into())
    }

    fn or_internal_server_error(self) -> Result<T> {
        self.ok_or_else(|| RESPONSE_ERROR_INTERNAL_SERVER_ERROR.clone().into())
    }

    fn or_not_found(self) -> Result<T> {
        self.ok_or_else(|| RESPONSE_ERROR_NOT_FOUND.clone().into())
    }

    fn or_unauthorized(self) -> Result<T> {
        self.ok_or_else(|| RESPONSE_ERROR_UNAUTHORIZED.clone().into())
    }
}

impl<T, E> OrHttpError<T> for Result<T, E> {
    fn or_bad_request(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(RESPONSE_ERROR_BAD_REQUEST.clone().into()),
        }
    }

    fn or_forbidden(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(RESPONSE_ERROR_FORBIDDEN.clone().into()),
        }
    }

    fn or_internal_server_error(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(RESPONSE_ERROR_INTERNAL_SERVER_ERROR.clone().into()),
        }
    }

    fn or_not_found(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(RESPONSE_ERROR_NOT_FOUND.clone().into()),
        }
    }

    fn or_unauthorized(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(RESPONSE_ERROR_UNAUTHORIZED.clone().into()),
        }
    }
}
