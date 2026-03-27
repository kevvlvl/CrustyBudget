use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use log::info;
use rust_decimal_macros::dec;
use serde::{Deserialize};
use crate::budget::calc::summarize_calc;
use crate::budget::db::{get, insert, update, EXPENSE_TABLE};
use crate::money_str;
use crate::types::budget_structs::{ItemDetails, Report};
use crate::types::enums::Frequency;

#[derive(Deserialize)]
pub struct ExpenseQuery {
    frequency: Frequency
}

pub async fn add_expense(Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("add_expense - Received expense payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("add_expense - Received expense with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    insert(&payload_str, EXPENSE_TABLE).expect("add_expense - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn update_expense(Path(id): Path<u64>, Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("update_expense - Received expense payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("update_expense - Received expense with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    update(id, &payload_str, EXPENSE_TABLE).expect("update_expense - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

fn get_positive_expense_filter() -> fn(&ItemDetails) -> bool {
    |i: &ItemDetails| i.amount.gt(&dec!(0))
}

pub async fn get_expense(expense_query: Query<ExpenseQuery>) -> Result<Json<Report>, StatusCode> {

    let expense_query: ExpenseQuery = expense_query.0;

    info!("get_expense - for frequency: {:?}", expense_query.frequency);

    let items_found = get::<_>(get_positive_expense_filter(), EXPENSE_TABLE)
        .map_err(|e| e.to_string());

    info!("get_expense - Found expense items: {:?}", items_found);

    let res = summarize_calc(&expense_query.frequency, items_found.unwrap());
    Ok(Json(res))
}