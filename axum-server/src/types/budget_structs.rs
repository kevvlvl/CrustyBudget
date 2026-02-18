use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::Frequency;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct IncomeEntry {
    pub amount: Decimal,
    pub source: Option<String>,
    pub frequency: Frequency,
    pub details: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ExpenseEntry {
    pub amount: Decimal,
    pub destination: Option<String>,
    pub frequency: Frequency,
    pub details: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct CreditCardExpenseEntry {
    pub amount: Decimal,
    pub payment_date: Option<NaiveDateTime>,
    pub details: Option<String>,
}