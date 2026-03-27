use axum::http::StatusCode;
use axum::{Json};
use axum::extract::{Path, Query};
use log::info;
use rust_decimal_macros::dec;
use serde::{Deserialize};
use crate::budget::calc::summarize_calc;
use crate::budget::db::{get, insert, update, INCOME_TABLE};
use crate::money_str;
use crate::types::budget_structs::{ItemDetails, Report};
use crate::types::enums::{Frequency};

#[derive(Deserialize)]
pub struct IncomeQuery {
    frequency: Frequency
}

pub async fn add_income(Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("add_income - Received income payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("add_income - Received income with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    insert(&payload_str, INCOME_TABLE).expect("add_income - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn update_income(Path(id): Path<u64>, Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("update_income - Received income payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("update_income - Received income with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    update(id, &payload_str, INCOME_TABLE).expect("update_income - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

fn get_positive_income_filter() -> fn(&ItemDetails) -> bool {
    |i: &ItemDetails| i.amount.gt(&dec!(0))
}

pub async fn get_income(income_query: Query<IncomeQuery>) -> Result<Json<Report>, StatusCode> {

    let income_query: IncomeQuery = income_query.0;

    info!("get_income - for frequency: {:?}", income_query.frequency);

    let items_found = get::<_>(get_positive_income_filter(), INCOME_TABLE)
        .map_err(|e| e.to_string());

    info!("get_income - Found income items: {:?}", items_found);

    let res = summarize_calc(&income_query.frequency, items_found.unwrap());
    Ok(Json(res))
}