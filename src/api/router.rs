use axum::Router;
use axum::routing::{get, post};
use log::{info};
use crate::budget;

const HEALTH_PATH: &str = "/api/health";
const INCOME_PATH: &str = "/api/budget/income";
const EXPENSE_PATH: &str = "/api/budget/expense";

async fn is_healthy() -> &'static str {
    "OK"
}

pub fn router() -> Router {

    info!("Initialize Router");

    Router::new()
        .route(HEALTH_PATH, get(is_healthy))
        .route(INCOME_PATH, get(budget::income::get_income))
        .route(INCOME_PATH, post(budget::income::define_income))
        .route(EXPENSE_PATH, get(budget::expense::get_expense))
        .route(EXPENSE_PATH, post(budget::expense::define_expense))
}