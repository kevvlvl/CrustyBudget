use axum::Router;
use axum::routing::{get, post};
use log::info;
use tower_http::cors::{Any, CorsLayer};
use crate::budget;

const HEALTH_PATH: &str = "/api/health";
const INCOME_PATH: &str = "/api/budget/income";
const EXPENSE_PATH: &str = "/api/budget/expense";

const CC_PATH: &str = "/api/budget/cc";

pub fn router() -> Router {

    info!("Initialize Router with cors support");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route(HEALTH_PATH, get(is_healthy))
        .route(INCOME_PATH, get(budget::income::get_income))
        .route(INCOME_PATH, post(budget::income::add_income))
        .route(EXPENSE_PATH, get(budget::expense::get_expense))
        .route(EXPENSE_PATH, post(budget::expense::add_expense))
        .route(CC_PATH, post(budget::cc::add_entry))
        .layer(cors)
}

async fn is_healthy() -> &'static str {
    "Is Healthy"
}