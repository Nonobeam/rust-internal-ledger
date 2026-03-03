use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer};

pub struct KafkaAdapter {
}

impl KafkaAdapter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_producer(bootstrap_server: &str) -> FutureProducer {
	    ClientConfig::new()
	        .set("bootstrap.servers", bootstrap_server)
	        .set("queue.buffering.max.ms", "0")
	        .create().expect("Failed to create client")
	}
}