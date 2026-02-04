use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use log::info;
use serde::{Deserialize};
use crate::budget::db::{save, EXPENSE_TABLE};
use crate::money_str;
use crate::types::budget_structs::ExpenseEntry;
use crate::types::enums::Frequency;



#[derive(Deserialize)]
pub struct ExpenseQuery {
    frequency: Frequency
}

pub async fn define_expense(Json(payload): Json<ExpenseEntry>) -> Result<Json<ExpenseEntry>, StatusCode> {

    info!("Received expense payload: {:?}", payload);

    let amount_str: String = money_str!("CAD", payload.amount);
    info!("Received expense with amount: {}", amount_str);

    let payload_str = serde_json::to_string(&payload).unwrap();
    save(&payload_str, EXPENSE_TABLE).expect("ERROR: Failed Writing to redb database");

    Ok(Json(payload))
}

pub async fn get_expense(expense_query: Query<ExpenseQuery>) -> Result<Json<ExpenseEntry>, StatusCode> {

    let expense_query: ExpenseQuery = expense_query.0;

    info!("Get expense for frequency: {:?}", expense_query.frequency);

    let result: ExpenseEntry = ExpenseEntry {
        amount: Default::default(),
        destination: None,
        frequency: expense_query.frequency,
        details: None,
    };

    Ok(Json(result))
}