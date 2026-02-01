use axum::http::StatusCode;
use axum::Json;
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::budget::db::{save, EXPENSE_TABLE, INCOME_TABLE};
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

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, EXPENSE_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}