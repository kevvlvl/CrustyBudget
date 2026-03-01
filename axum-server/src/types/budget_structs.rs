use chrono::{NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::{ExpenseCategory, Frequency};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Identifier<T> {
    pub id: u64,
    pub value: T,
}

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
    pub category: Option<ExpenseCategory>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct CreditCardExpenseEntry {
    pub created_date: Option<NaiveDate>,
    pub name: String,
    pub amount: Decimal,
    pub due_date: NaiveDate,
    pub amount_paid: Decimal,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryIncomeReport {
    pub frequency: Frequency,
    pub items: Vec<SummaryIncomeReportItem>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryIncomeReportItem {
    pub id: u64,
    pub amount: Decimal,
    pub details: IncomeEntry,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryExpenseReport {
    pub frequency: Frequency,
    pub items: Vec<SummaryExpenseReportItem>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryExpenseReportItem {
    pub id: u64,
    pub amount: Decimal,
    pub details: ExpenseEntry,
}

impl PartialEq for SummaryIncomeReportItem {
    fn eq(&self, other: &Self) -> bool {
        self.amount.eq(&other.amount) && self.details == other.details
    }
}

impl PartialEq for SummaryExpenseReportItem {
    fn eq(&self, other: &Self) -> bool {
        self.amount.eq(&other.amount) && self.details == other.details
    }
}

impl PartialEq for IncomeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.amount.eq(&other.amount) && self.frequency == other.frequency && self.details == other.details
    }
}

impl PartialEq for ExpenseEntry {
    fn eq(&self, other: &Self) -> bool {
        self.destination == other.destination && self.amount.eq(&other.amount) && self.frequency == other.frequency && self.details == other.details
    }
}