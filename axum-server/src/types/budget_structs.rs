use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::types::enums::{ExpenseCategory, Frequency, IncomeCategory};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Report {
    pub items: Vec<AggregatedItemDetails>,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Identifier {
    pub id: u64,
    pub item_details: ItemDetails,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ItemDetails {
    pub amount: Decimal,
    pub expense_category: Option<ExpenseCategory>,
    pub expense_due_date: Option<NaiveDate>,
    pub frequency: Frequency,
    pub income_category: Option<IncomeCategory>,
    pub name: String,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AggregatedItemDetails {
    pub calculated_amount: Decimal,
    pub frequency: Frequency,
    pub item: Identifier,
}

impl PartialEq for ItemDetails {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.amount.eq(&other.amount) &&
            self.frequency == other.frequency &&
            self.income_category == other.income_category &&
            self.expense_category == other.expense_category
    }
}
