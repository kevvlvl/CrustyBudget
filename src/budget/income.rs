use axum::http::StatusCode;
use axum::{Json};
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::budget::db::{save, INCOME_TABLE};
use crate::types::enums::{Frequency};

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

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, INCOME_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}
