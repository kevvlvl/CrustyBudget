use std::ops::{Div, Mul};
use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::types::budget_structs::{FinancialEntry, SummaryReport};
use crate::types::enums::Frequency;

/**
Return a dollar amount with the specified currency, with two decimals
**/
#[macro_export] macro_rules! money_str  {
    ($currency:expr, $val:expr) => {
        format!("${} {:.2}", $currency, $val)
    }
}

pub fn summarize_calc<T: FinancialEntry>(frequency: &Frequency, entries: Vec<T>) -> SummaryReport {

    let mut summary_income: SummaryReport = SummaryReport {
        amount: Default::default(),
        details: Some("Summary of filtered incomes".to_string()),
    };

    for entry in entries {
        info!("Current entry: {}", entry.get_details());
        summary_income.amount += get_frequency_amount(frequency, &entry.get_frequency(), entry.get_amount());
    }

    summary_income
}

/**
Return the amount for the desired target frequency.
ASSUMPTIONS FOR SIMPLIFICATION: 1 month contains 31 days, biweekly means twice a month
Personal project disclaimer: This is not exactly accurate okay!
**/
fn get_frequency_amount(target_frequency: &Frequency, amount_frequency: &Frequency, amount: Decimal) -> Decimal {

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

    res.round_dp(4)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_daily() {

        let res = get_frequency_amount(&Frequency::Daily, &Frequency::Biweekly, dec!(2800));
        assert_eq!(res, dec!(200));
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_weekly() {

        let res = get_frequency_amount(&Frequency::Weekly, &Frequency::Biweekly, dec!(333));
        assert_eq!(res, dec!(166.50));
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_biweekly() {

        let res = get_frequency_amount(&Frequency::Biweekly, &Frequency::Biweekly, dec!(333));
        assert_eq!(res, dec!(333));
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_monthly() {

        let res = get_frequency_amount(&Frequency::Monthly, &Frequency::Biweekly, dec!(500));
        assert_eq!(res, dec!(1000));
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_daily() {

        let res = get_frequency_amount(&Frequency::Daily, &Frequency::Weekly, dec!(1400));
        assert_eq!(res, dec!(200));
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_weekly() {

        let res = get_frequency_amount(&Frequency::Weekly, &Frequency::Weekly, dec!(222));
        assert_eq!(res, dec!(222));
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_biweekly() {

        let res = get_frequency_amount(&Frequency::Biweekly, &Frequency::Weekly, dec!(222));
        assert_eq!(res, dec!(444));
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_monthly() {

        let res = get_frequency_amount(&Frequency::Monthly, &Frequency::Weekly, dec!(222));
        assert_eq!(res, dec!(888));
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_daily() {

        let res = get_frequency_amount(&Frequency::Daily, &Frequency::Daily, dec!(50));
        assert_eq!(res, dec!(50));
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_weekly() {

        let res = get_frequency_amount(&Frequency::Weekly, &Frequency::Daily, dec!(5));
        assert_eq!(res, dec!(35));
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_biweekly() {

        let res = get_frequency_amount(&Frequency::Biweekly, &Frequency::Daily, dec!(5));
        assert_eq!(res, dec!(70));
    }
    #[test]
    fn test_get_frequency_amount_from_daily_to_monthly() {

        let res = get_frequency_amount(&Frequency::Monthly, &Frequency::Daily, dec!(5));
        assert_eq!(res, dec!(155));
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_daily() {

        let res = get_frequency_amount(&Frequency::Daily, &Frequency::Monthly, dec!(2500));
        assert_eq!(res,  dec!(80.6452));
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_weekly() {

        let res = get_frequency_amount(&Frequency::Weekly, &Frequency::Monthly, dec!(2500));
        assert_eq!(res,  dec!(625));
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_biweekly() {

        let res = get_frequency_amount(&Frequency::Biweekly, &Frequency::Monthly, dec!(2500));
        assert_eq!(res,  dec!(1250));
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_monthly() {

        let res = get_frequency_amount(&Frequency::Monthly, &Frequency::Monthly, dec!(1234));
        assert_eq!(res,  dec!(1234));
    }
}