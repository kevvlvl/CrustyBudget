use std::env;
use axum::{serve, Router};
use axum::routing::{get, post};
use log::info;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod budget;
mod types;

const HEALTH_PATH: &str = "/api/health";
const INCOME_PATH: &str = "/api/budget/income";
const EXPENSE_PATH: &str = "/api/budget/expense";

#[tokio::main(flavor = "current_thread")]
async fn main() {

    env_logger::init();

    info!("Starting Crusty Budget Service");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let r = Router::new()
        .route(HEALTH_PATH, get(is_healthy))
        .route(INCOME_PATH, get(budget::income::get_income))
        .route(INCOME_PATH, post(budget::income::define_income))
        .route(EXPENSE_PATH, get(budget::expense::get_expense))
        .route(EXPENSE_PATH, post(budget::expense::define_expense))
        .layer(cors);

    let addr = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("{}:{}", addr, port)).await.unwrap();

    serve(listener, r).await.unwrap()
}

async fn is_healthy() -> &'static str {
    "Is Healthy"
}