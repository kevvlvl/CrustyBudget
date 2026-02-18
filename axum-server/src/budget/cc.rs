use axum::http::StatusCode;
use axum::Json;
use chrono::{NaiveDate};
use log::info;
use crate::budget::db::{save, CC_TABLE};
use crate::money_str;
use crate::types::budget_structs::{CreditCardExpenseEntry};

pub async fn add_entry(Json(payload): Json<CreditCardExpenseEntry>) -> Result<Json<CreditCardExpenseEntry>, StatusCode> {

    info!("Received cc payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    let due_date: NaiveDate = payload.payment_date.unwrap();
    info!("Received cc with amount: {} due on date {}", amount_str, due_date);

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, CC_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}