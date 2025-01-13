use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0} not found")]
    NotFound(String),

    #[error("{0}")]
    ConvertError(String),

    #[error("{0}")]
    GetRateError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ConvertError(_) => StatusCode::BAD_REQUEST,
            AppError::GetRateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, format!("{:?}", self)).into_response()
    }
}
