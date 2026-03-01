use chrono::{NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::{ExpenseCategory, Frequency};

pub trait FinancialEntry {
    fn get_amount(&self) -> Decimal;
    fn get_name(&self) -> String;
    fn get_frequency(&self) -> Frequency;

    fn get_category(&self) -> String;
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
pub struct SummaryReport {
    pub frequency: Frequency,
    pub items: Vec<SummaryReportItem>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryReportItem {
    pub amount: Decimal,
    pub category: String,
    pub name: String,
}

impl FinancialEntry for IncomeEntry {
    fn get_amount(&self) -> Decimal {
        self.amount
    }

    fn get_name(&self) -> String {
        self.source.clone().unwrap()
    }

    fn get_frequency(&self) -> Frequency {
        self.frequency.clone()
    }

    fn get_category(&self) -> String {
        self.source.clone().unwrap()
    }
}

impl FinancialEntry for ExpenseEntry {
    fn get_amount(&self) -> Decimal {
        self.amount
    }

    fn get_name(&self) -> String {
        self.destination.clone().unwrap()
    }

    fn get_frequency(&self) -> Frequency {
        self.frequency.clone()
    }

    fn get_category(&self) -> String {
        self.destination.clone().unwrap()
    }
}

impl PartialEq for SummaryReportItem {
    fn eq(&self, other: &Self) -> bool {
        self.amount.eq(&other.amount) && self.name == other.name
    }
}