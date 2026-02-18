use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use log::info;
use rust_decimal_macros::dec;
use serde::{Deserialize};
use crate::budget::calc::{summarize_calc};
use crate::budget::db::{get, save, EXPENSE_TABLE};
use crate::money_str;
use crate::types::budget_structs::{ExpenseEntry, SummaryReport};
use crate::types::enums::Frequency;

#[derive(Deserialize)]
pub struct ExpenseQuery {
    frequency: Frequency
}

pub async fn add_expense(Json(payload): Json<ExpenseEntry>) -> Result<Json<ExpenseEntry>, StatusCode> {

    info!("Received expense payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("Received expense with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, EXPENSE_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn get_expense(expense_query: Query<ExpenseQuery>) -> Result<Json<SummaryReport>, StatusCode> {

    let expense_query: ExpenseQuery = expense_query.0;

    info!("Get expense for frequency: {:?}", expense_query.frequency);

    let items_filter = |i: &ExpenseEntry| i.amount.gt(&dec!(0));
    let items_found = get::<_, ExpenseEntry>(items_filter, EXPENSE_TABLE)
        .map_err(|e| e.to_string());

    info!("Get expense for items: {:?}", items_found);

    let res = summarize_calc(&expense_query.frequency, items_found.unwrap());
    Ok(Json(res))
}