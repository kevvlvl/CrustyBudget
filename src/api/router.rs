use axum::Router;
use axum::routing::{get, post};
use log::{info};
use crate::budget;

async fn is_healthy() -> &'static str {
    "OK"
}

pub fn router() -> Router {

    info!("Initialize Router");

    Router::new()
        .route("/api/health", get(is_healthy))
        .route("/api/budget/income", post(budget::income::define_income))
        .route("/api/budget/expense", post(budget::expense::define_expense))
}