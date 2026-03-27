use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum Frequency {
    Once,
    Daily,
    Weekly,
    Biweekly,
    Monthly
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum IncomeCategory {
    Salary,
    Investments,
    Other
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum ExpenseCategory {
    Entertainment,
    Food,
    Housing,
    Transportation,
    OnlineServices,
    Utilities,
    Misc
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match  *self {
            Frequency::Once => write!(f, "Once"),
            Frequency::Daily => write!(f, "Daily"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Biweekly => write!(f, "Biweekly"),
            Frequency::Monthly => write!(f, "Monthly"),
        }
    }
}