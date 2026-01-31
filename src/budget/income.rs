use axum::http::StatusCode;
use axum::Json;
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::Frequency;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct IncomeEntry {
    amount: Decimal,
    source: String,
    frequency: Frequency,
    details: Option<String>,
}

pub async fn define_income(Json(payload): Json<IncomeEntry>) -> Result<Json<IncomeEntry>, StatusCode> {

    info!("Received income payload: {:?}", payload);

    Ok(Json(payload))
}