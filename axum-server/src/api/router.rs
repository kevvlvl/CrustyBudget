use axum::Router;
use axum::routing::{get, post, put};
use log::info;
use tower_http::cors::{Any, CorsLayer};
use crate::budget;

const HEALTH_PATH: &str = "/api/health";
const INCOME_PATH: &str = "/api/budget/income";
const INCOME_PATH_PUT: &str = "/api/budget/income/{id}";
const EXPENSE_PATH: &str = "/api/budget/expense";
const EXPENSE_PATH_PUT: &str = "/api/budget/expense/{id}";
const CC_PATH: &str = "/api/budget/cc";
const CC_PATH_PUT: &str = "/api/budget/cc/{id}";

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
        .route(INCOME_PATH_PUT, put(budget::income::update_income))
        .route(EXPENSE_PATH, get(budget::expense::get_expense))
        .route(EXPENSE_PATH, post(budget::expense::add_expense))
        .route(EXPENSE_PATH_PUT, put(budget::expense::update_expense))
        .route(CC_PATH, get(budget::cc::get_credits))
        .route(CC_PATH, post(budget::cc::add_cc))
        .route(CC_PATH_PUT, put(budget::cc::update_cc))
        .layer(cors)
}

async fn is_healthy() -> &'static str {
    "Is Healthy"
}