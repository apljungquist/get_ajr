use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::{error, warn};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid path: {0}")]
    InvalidPath(anyhow::Error),

    #[error("Invalid query: {0}")]
    InvalidQuery(anyhow::Error),

    #[error("Internal error: {0}")]
    Internal(anyhow::Error),

    #[error("Other error: {0}")]
    Other(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::InvalidQuery(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidPath(_) => StatusCode::BAD_REQUEST,
            AppError::Other(_) => StatusCode::BAD_REQUEST,
        };

        if status.is_server_error() {
            error!("Server error: {self:?}");
        } else {
            warn!("Client error: {self:?}");
        }

        (status, self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
