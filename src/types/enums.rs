use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly
}