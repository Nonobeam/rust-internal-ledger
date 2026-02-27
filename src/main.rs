mod config;
mod domain;
mod service;
mod repository;
mod common;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
}