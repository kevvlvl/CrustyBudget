use std::ops::{Div, Mul};
use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::types::budget_structs::IncomeEntry;
use crate::types::enums::Frequency;

/**
Return a dollar amount with the specified currency, with two decimals
**/
#[macro_export] macro_rules! money_str  {
    ($currency:expr, $val:expr) => {
        format!("${} {:.2}", $currency, $val)
    }
}

pub fn summarize_income(frequency: &Frequency, incomes: Vec<IncomeEntry>) -> IncomeEntry {

    let mut summary_income: IncomeEntry = IncomeEntry {
        amount: Default::default(),
        source: None,
        frequency: frequency.clone(),
        details: Some("Summary of filtered incomes".to_string()),
    };

    for income in incomes {
        info!("Current income {:?}", income);
        summary_income.amount += get_frequency_income(&frequency, income.frequency, income.amount);
    }

    summary_income
}

/**
Return the amount for the desired target frequency.
ASSUMPTIONS FOR SIMPLIFICATION: 1 month contains 31 days, biweekly means twice a month
Personal project disclaimer: This is not exactly accurate okay!
**/
fn get_frequency_income(target_frequency: &Frequency, amount_frequency: Frequency, amount: Decimal) -> Decimal {

    let res: Decimal;
    match target_frequency {
        Frequency::Monthly => {
            match amount_frequency {
                Frequency::Monthly => {
                    res = amount
                }
                Frequency::Daily => {
                    res = amount.mul(dec!(31))
                }
                Frequency::Weekly => {
                    res = amount.mul(dec!(4))
                }
                Frequency::Biweekly => {
                    res = amount.mul(dec!(2))
                }
            }
        }
        Frequency::Daily => {
            match amount_frequency {
                Frequency::Monthly => {
                    res = amount.div(dec!(31))
                }
                Frequency::Daily => {
                    res = amount
                }
                Frequency::Weekly => {
                    res = amount.div(dec!(7))
                }
                Frequency::Biweekly => {
                    res = amount.div(dec!(14))
                }
            }
        }
        Frequency::Weekly => {
            match amount_frequency {
                Frequency::Monthly => {
                    res = amount.div(dec!(4))
                }
                Frequency::Daily => {
                    res = amount.mul(dec!(7))
                }
                Frequency::Weekly => {
                    res = amount
                }
                Frequency::Biweekly => {
                    res = amount.div(dec!(2));
                }
            }
        }
        Frequency::Biweekly => {
            match amount_frequency {
                Frequency::Monthly => {
                    res = amount.div(dec!(2))
                }
                Frequency::Daily => {
                    res = amount.mul(dec!(14))
                }
                Frequency::Weekly => {
                    res = amount.mul(dec!(2));
                }
                Frequency::Biweekly => {
                    res = amount
                }
            }
        }
    }

    res
}