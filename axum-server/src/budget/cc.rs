use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use log::info;
use serde::Deserialize;
use crate::budget::db::{get, insert, update, CC_TABLE};
use crate::money_str;
use crate::types::budget_structs::{ItemDetails, Identifier};

#[derive(Deserialize)]
pub struct CreditCardQuery {
    from_date: NaiveDate
}

pub async fn add_cc(Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("add_cc - Received cc payload: {:?}", payload);

    let name: &str = &payload.name;
    let amount_str: &str  = &money_str!("CAD", payload.amount);
    info!("add_cc - Received cc {} with amount: {}", name, amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    insert(&payload_str, CC_TABLE).expect("add_cc - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn update_cc(Path(id): Path<u64>, Json(payload): Json<ItemDetails>) -> Result<Json<ItemDetails>, StatusCode> {

    info!("update_cc - Received cc payload: {:?}", payload);

    let name: &str = &payload.name;
    let amount_str: &str  = &money_str!("CAD", payload.amount);
    info!("update_cc - Received cc {} with amount: {}", name, amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    update(id, &payload_str, CC_TABLE).expect("update_cc - ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

fn get_items_due_after_date_filter(from_date: &NaiveDate) -> impl Fn(&ItemDetails) -> bool {
    move |i: &ItemDetails| i.expense_due_date.unwrap_or(NaiveDate::MIN).gt(from_date)
}

pub async fn get_credits(credit_card_query: Query<CreditCardQuery>) -> Result<Json<Vec<Identifier>>, StatusCode> {

    let credit_card_query: CreditCardQuery = credit_card_query.0;

    info!("get_credits - Get expense for frequency: {:?}", credit_card_query.from_date);

    let items_found = get::<_>(get_items_due_after_date_filter(&credit_card_query.from_date), CC_TABLE)
        .map_err(|e| e.to_string()).unwrap();

    info!("get_credits - Found credit card items: {:?}", items_found);

    Ok(Json(items_found))
}