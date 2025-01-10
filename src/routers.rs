use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{errors::AppError, state::AppState};

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

pub async fn get_router(state: AppState) -> Result<axum::Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/convert", post(convert_handler))
        .with_state(state.clone());

    Ok(api_router)
}

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn convert_handler(
    State(state): State<AppState>,
    Json(req): Json<CurrencyConvertRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !state.symbols.contains_key(&req.from_unit) || !state.symbols.contains_key(&req.to) {
        return Err(AppError::NotFound("Currency not supported".to_string()));
    }

    let rate = state.exchange_rate.get(&req.from_unit);
    if rate.is_none() {
        return Err(AppError::NotFound("rate not found".to_string()));
    }

    let resp = CurrencyConvertResponse {
        result: req.from_value * rate.unwrap().rate,
        unit: req.to,
    };

    Ok((StatusCode::OK, Json(resp)))
}
