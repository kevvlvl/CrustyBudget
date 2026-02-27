use std::ops::{Div, Mul};
use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::types::budget_structs::{FinancialEntry, SummaryReport, SummaryReportItem};
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
        frequency: frequency.clone(),
        items: vec![],
    };

    for entry in entries {
        info!("Current entry: name {} - category {} - frequency {} - amount {} ",
            entry.get_name(),
            entry.get_category(),
            entry.get_frequency(),
            entry.get_amount());

        summary_income.items.push(get_frequency_amount(frequency, &entry.get_name(), &entry.get_category(), &entry.get_frequency(), entry.get_amount()));

    }

    summary_income
}

/**
Return the amount for the desired target frequency.
ASSUMPTIONS FOR SIMPLIFICATION: 1 month contains 31 days, biweekly means twice a month
Personal project disclaimer: This is not exactly accurate okay!
**/
fn get_frequency_amount(target_frequency: &Frequency, name: &str, category: &str, amount_frequency: &Frequency, amount: Decimal) -> SummaryReportItem {

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

    SummaryReportItem {
        name: name.to_string(),
        category: category.to_string(),
        amount: res.round_dp(4)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_daily() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(200)
        };

        let res = get_frequency_amount(&Frequency::Daily, &expected_item.name, &expected_item.category, &Frequency::Biweekly, dec!(2800));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_weekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(166.50)
        };

        let res = get_frequency_amount(&Frequency::Weekly, &expected_item.name, &expected_item.category, &Frequency::Biweekly, dec!(333));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_biweekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(333)
        };

        let res = get_frequency_amount(&Frequency::Biweekly, &expected_item.name, &expected_item.category, &Frequency::Biweekly, dec!(333));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_biweekly_to_monthly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(1000)
        };

        let res = get_frequency_amount(&Frequency::Monthly, &expected_item.name, &expected_item.category, &Frequency::Biweekly, dec!(500));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_daily() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(200)
        };

        let res = get_frequency_amount(&Frequency::Daily, &expected_item.name, &expected_item.category, &Frequency::Weekly, dec!(1400));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_weekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(222)
        };

        let res = get_frequency_amount(&Frequency::Weekly, &expected_item.name, &expected_item.category, &Frequency::Weekly, dec!(222));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_biweekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(444)
        };

        let res = get_frequency_amount(&Frequency::Biweekly, &expected_item.name, &expected_item.category, &Frequency::Weekly, dec!(222));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_weekly_to_monthly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(888)
        };

        let res = get_frequency_amount(&Frequency::Monthly, &expected_item.name, &expected_item.category, &Frequency::Weekly, dec!(222));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_daily() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(50)
        };

        let res = get_frequency_amount(&Frequency::Daily, &expected_item.name, &expected_item.category, &Frequency::Daily, dec!(50));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_weekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(35)
        };

        let res = get_frequency_amount(&Frequency::Weekly, &expected_item.name, &expected_item.category, &Frequency::Daily, dec!(5));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_daily_to_biweekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(70)
        };

        let res = get_frequency_amount(&Frequency::Biweekly, &expected_item.name, &expected_item.category, &Frequency::Daily, dec!(5));
        assert_eq!(res, expected_item);
    }
    #[test]
    fn test_get_frequency_amount_from_daily_to_monthly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(155)
        };

        let res = get_frequency_amount(&Frequency::Monthly, &expected_item.name, &expected_item.category, &Frequency::Daily, dec!(5));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_daily() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(80.6452)
        };

        let res = get_frequency_amount(&Frequency::Daily, &expected_item.name, &expected_item.category, &Frequency::Monthly, dec!(2500));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_weekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(625)
        };

        let res = get_frequency_amount(&Frequency::Weekly, &expected_item.name, &expected_item.category, &Frequency::Monthly, dec!(2500));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_biweekly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(1250)
        };

        let res = get_frequency_amount(&Frequency::Biweekly, &expected_item.name, &expected_item.category, &Frequency::Monthly, dec!(2500));
        assert_eq!(res, expected_item);
    }

    #[test]
    fn test_get_frequency_amount_from_monthly_to_monthly() {

        let expected_item = SummaryReportItem {
            name: "test-item".to_string(),
            category: "Utilities".to_string(),
            amount: dec!(1234)
        };

        let res = get_frequency_amount(&Frequency::Monthly, &expected_item.name, &expected_item.category, &Frequency::Monthly, dec!(1234));
        assert_eq!(res, expected_item);
    }
}