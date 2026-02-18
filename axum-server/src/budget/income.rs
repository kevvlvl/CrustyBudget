use axum::http::StatusCode;
use axum::{Json};
use axum::extract::Query;
use log::info;
use rust_decimal_macros::dec;
use serde::{Deserialize};
use crate::budget::calc::summarize_calc;
use crate::budget::db::{get, save, INCOME_TABLE};
use crate::money_str;
use crate::types::budget_structs::{IncomeEntry, SummaryReport};
use crate::types::enums::{Frequency};

#[derive(Deserialize)]
pub struct IncomeQuery {
    frequency: Frequency
}

pub async fn add_income(Json(payload): Json<IncomeEntry>) -> Result<Json<IncomeEntry>, StatusCode> {

    info!("Received income payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("Received income with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, INCOME_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn get_income(income_query: Query<IncomeQuery>) -> Result<Json<SummaryReport>, StatusCode> {

    let income_query: IncomeQuery = income_query.0;

    info!("Get income for frequency: {:?}", income_query.frequency);

    let items_filter = |i: &IncomeEntry| i.amount.gt(&dec!(0));
    let items_found = get::<_, IncomeEntry>(items_filter, INCOME_TABLE)
        .map_err(|e| e.to_string());

    info!("Get income for items: {:?}", items_found);

    let res = summarize_calc(&income_query.frequency, items_found.unwrap());
    Ok(Json(res))
}