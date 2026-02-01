
/**
Return a dollar amount with the specified currency, with two decimals
**/
#[macro_export] macro_rules! money_str  {
    ($currency:expr, $val:expr) => {
        format!("${} {:.2}", $currency, $val)
    }
}