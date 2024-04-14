use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DbErr;
use std::{
    error::Error,
    fmt::{Debug, Display},
    num::ParseIntError,
};
use utoipa::ToSchema;

use crate::services::chat::ServiceError;

#[derive(thiserror::Error)]
pub enum AppError {
    AuthorizationHeaderMissing,
    AuthorizationHeaderBadChars,
    AuthorizationHeaderBadSchema,
    Unauthorized,
    Forbidden,
    InternalServerError(Box<dyn Error>),
    JwtError(Box<dyn Error>),
    UserAlreadyBlacklisted,
    UserWasNotFound(i64),
    UserNotBlacklisted,
    BadRequest(Details),
    UrlAlreadyExists,
    VideoReviewIdNotFound,
    SymbolNotFound,
    OrderWasNotFound,
    OrderAlreadySucceeded,
    AuthUserDenied,
    BadAuthQuery,
    AuthRequestFailed,
    AuthBadResponse,
    EmptyCredentials,
    LoginOccupied,
    AdminNotFound,
    ModeratorIsAdmin,
    ModeratorAlreadyAssigned,
    ModeratorNotAssigned,
    OrderIsCompletedOrCancelled,
    ReviewWasNotFound,
    OrderAlreadyCanceled,
    SymbolAlreadyExists,
    NameWasNotFound,
    #[error(transparent)]
    ParseError(#[from] ParseIntError),
    #[error(transparent)]
    ChatServiceError(#[from] ServiceError),
    #[error(transparent)]
    DbErr(#[from] DbErr),
    RequisitesWereNotFound,
    ChatWasNotFound,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::AuthorizationHeaderMissing => write!(f, "Authorization header missing"),
            AppError::AuthorizationHeaderBadChars => write!(f, "Authorization header bad chars"),
            AppError::AuthorizationHeaderBadSchema => write!(f, "Authorization header bad schema"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::InternalServerError(cause) => {
                tracing::error!(%cause, "Error!");
                write!(f, "Some error occurred on the server!")
            }
            AppError::JwtError(cause) => {
                tracing::error!(%cause, "jwt error!");
                write!(f, "Bad or expired token!")
            }
            AppError::Forbidden => write!(f, "You do not have enough permissions"),
            AppError::UserAlreadyBlacklisted => write!(f, "User has already been blacklisted"),
            AppError::UserWasNotFound(id) => write!(f, "User with id = {} was not found", id),
            AppError::UserNotBlacklisted => write!(f, "User is not blacklisted"),
            AppError::BadRequest(details) => write!(f, "{}", details.details),
            AppError::UrlAlreadyExists => {
                write!(f, "Provided url has already been added to video reviews")
            }
            AppError::VideoReviewIdNotFound => {
                write!(f, "Video review with provided id was not found")
            }
            AppError::SymbolNotFound => {
                write!(f, "Currency symbol was not found")
            }
            AppError::OrderWasNotFound => write!(f, "Order was not found"),
            AppError::OrderAlreadySucceeded => {
                write!(f, "Order has already been marked as succeeded")
            }
            AppError::AuthUserDenied => write!(f, "Auth was denied for this user"),
            AppError::BadAuthQuery => write!(f, "Bad auth query"),
            AppError::AuthRequestFailed => write!(f, "Auth request failed"),
            AppError::AuthBadResponse => write!(f, "Auth received bad response"),
            AppError::EmptyCredentials => write!(f, "Login and password cant be empty"),
            AppError::LoginOccupied => write!(f, "Provided login was already occupied"),
            AppError::AdminNotFound => write!(f, "Admin or moderator was not found"),
            AppError::ModeratorIsAdmin => write!(f, "Moderator has admin role"),
            AppError::ModeratorAlreadyAssigned => {
                write!(f, "Moderator has already been assigned to this order")
            }
            AppError::ModeratorNotAssigned => write!(f, "Moderator is not assigned to this order"),
            AppError::OrderIsCompletedOrCancelled => write!(f, "Order is completed or cancelled"),
            AppError::ReviewWasNotFound => write!(f, "Review was not found"),
            AppError::OrderAlreadyCanceled => {
                write!(f, "Order has already been marked as canceled")
            }
            AppError::SymbolAlreadyExists => write!(f, "Symbol already exists"),
            AppError::NameWasNotFound => write!(f, "Name was not found"),
            AppError::ChatServiceError(error) => write!(f, "{}", error),
            AppError::ParseError(error) => write!(f, "Failed to parse string to number. {}", error),
            AppError::DbErr(error) => write!(f, "{}", error),
            AppError::RequisitesWereNotFound => write!(f, "Requisites were not found"),
            AppError::ChatWasNotFound => write!(f, "Chat was not found"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Details {
    pub details: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let response = (
            Into::<StatusCode>::into(&self),
            Json(Details {
                details: format!("{:?}", self),
            }),
        );
        tracing::error!(cause = response.1.details, "Response with error!");
        response.into_response()
    }
}

impl From<&AppError> for StatusCode {
    fn from(val: &AppError) -> Self {
        match val {
            AppError::AuthorizationHeaderMissing => StatusCode::BAD_REQUEST,
            AppError::AuthorizationHeaderBadChars => StatusCode::BAD_REQUEST,
            AppError::AuthorizationHeaderBadSchema => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::UserAlreadyBlacklisted => StatusCode::CONFLICT,
            AppError::UserWasNotFound(_) => StatusCode::NOT_FOUND,
            AppError::UserNotBlacklisted => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::UrlAlreadyExists => StatusCode::CONFLICT,
            AppError::VideoReviewIdNotFound => StatusCode::NOT_FOUND,
            AppError::SymbolNotFound => StatusCode::NOT_FOUND,
            AppError::OrderWasNotFound => StatusCode::NOT_FOUND,
            AppError::OrderAlreadySucceeded => StatusCode::BAD_REQUEST,
            AppError::AuthUserDenied => StatusCode::FORBIDDEN,
            AppError::BadAuthQuery => StatusCode::BAD_REQUEST,
            AppError::AuthRequestFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthBadResponse => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EmptyCredentials => StatusCode::BAD_REQUEST,
            AppError::LoginOccupied => StatusCode::CONFLICT,
            AppError::AdminNotFound => StatusCode::NOT_FOUND,
            AppError::ModeratorIsAdmin => StatusCode::FORBIDDEN,
            AppError::ModeratorAlreadyAssigned => StatusCode::CONFLICT,
            AppError::ModeratorNotAssigned => StatusCode::CONFLICT,
            AppError::OrderIsCompletedOrCancelled => StatusCode::CONFLICT,
            AppError::ReviewWasNotFound => StatusCode::NOT_FOUND,
            AppError::OrderAlreadyCanceled => StatusCode::CONFLICT,
            AppError::SymbolAlreadyExists => StatusCode::CONFLICT,
            AppError::NameWasNotFound => StatusCode::NOT_FOUND,
            AppError::ChatServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            AppError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RequisitesWereNotFound => StatusCode::NOT_FOUND,
            AppError::ChatWasNotFound => StatusCode::NOT_FOUND,
        }
    }
}
