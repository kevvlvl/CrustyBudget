use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly
}