use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{errors::AppError, rate::RateGetter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyConvertRequest {
    pub from_value: f64,
    pub from_unit: String, // YUAN
    pub to: String,        // USD
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyConvertResponse {
    pub result: f64,
    pub unit: String,
}

pub async fn get_router() -> Result<axum::Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/convert", post(convert_handler));

    Ok(api_router)
}

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn convert_handler(
    Json(req): Json<CurrencyConvertRequest>,
) -> Result<impl IntoResponse, AppError> {
    let rate_getter = RateGetter::new(req.from_unit.clone().to_lowercase());
    let exchange_rate = rate_getter.get_exc_rate().await?;
    let rate = exchange_rate.get_rate(&req.to.to_lowercase());
    if rate.is_none() {
        return Err(AppError::ConvertError(format!(
            "currency: {} not found",
            req.to
        )));
    }

    let resp = CurrencyConvertResponse {
        result: req.from_value * rate.unwrap(),
        unit: req.to.clone(),
    };

    Ok((StatusCode::OK, Json(resp)))
}
