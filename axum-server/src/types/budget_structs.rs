use chrono::{NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::{ExpenseCategory, Frequency};

pub trait FinancialEntry {
    fn get_amount(&self) -> Decimal;
    fn get_details(&self) -> String;
    fn get_frequency(&self) -> Frequency;
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
    pub amount: Decimal,
    pub payment_date: Option<NaiveDate>,
    pub details: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SummaryReport {
    pub amount: Decimal,
    pub details: Option<String>,
}

impl FinancialEntry for IncomeEntry {
    fn get_amount(&self) -> Decimal {
        self.amount
    }

    fn get_details(&self) -> String {

        let src = self.source.clone().unwrap();
        let d = self.details.clone().unwrap();
        format!("source : {} - details: {}", src, d)
    }

    fn get_frequency(&self) -> Frequency {
        self.frequency.clone()
    }
}

impl FinancialEntry for ExpenseEntry {
    fn get_amount(&self) -> Decimal {
        self.amount
    }

    fn get_details(&self) -> String {

        let dst = self.destination.clone().unwrap();
        let d = self.details.clone().unwrap();
        format!("destination : {} - details: {}", dst, d)
    }

    fn get_frequency(&self) -> Frequency {
        self.frequency.clone()
    }
}