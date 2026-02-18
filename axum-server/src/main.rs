use std::env;
use axum::{serve};
use log::info;
use tokio::net::TcpListener;

mod budget;
mod types;
mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() {

    env_logger::init();
    let addr = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    info!("Starting Crusty Budget Service");

    let r = api::router::router();
    let listener = TcpListener::bind(format!("{}:{}", addr, port)).await.unwrap();

    serve(listener, r).await.unwrap()
}