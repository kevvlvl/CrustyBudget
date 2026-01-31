use axum::http::StatusCode;
use axum::Json;
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::Frequency;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ExpenseEntry {
    amount: Decimal,
    destination: String,
    frequency: Frequency,
    details: Option<String>,
}

pub async fn define_expense(Json(payload): Json<ExpenseEntry>) -> Result<Json<ExpenseEntry>, StatusCode> {

    info!("Received expense payload: {:?}", payload);

    Ok(Json(payload))
}