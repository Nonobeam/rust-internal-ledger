mod config;
mod domain;
mod service;
mod traits;
mod common;
mod adapters;

use std::env::args;
use dotenv::dotenv;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::adapters::kafka::KafkaAdapter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
 
    stdout.write(b"Welcome to Kafka chat!\n").await.unwrap();

    let producer = KafkaAdapter::create_producer(&args().skip(1).next()
        .unwrap_or("localhost:9092".to_string()));
 
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
 
    loop {
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();
 
        match input_lines.next_line().await.unwrap() {
            Some(line) => {
                producer.send(FutureRecord::<(), _>::to("ledger")
                  .payload(&line), Timeout::Never)
                    .await
                    .expect("Failed to produce");
            }
            None => break,
        }
    }
}