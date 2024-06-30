use std::env;

use ene_kafka::kafka_admin;
use ene_kafka::admins::KafkaAdminInterface;

/// This examples demonstrates how the Kafka Admin can be used
#[tokio::main]
async fn main() -> ene_kafka::KafkaResult<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let bootstrap_servers = "localhost:9092".to_string();

    let admin = kafka_admin!(bootstrap_servers = bootstrap_servers, request_time_out_ms = "50000".to_string(), connection_max_idle_ms = "0".to_string());
    
    let is_topic_live = admin.check_topic_liveness("test").await?;
    println!("Is topic live: {}", is_topic_live);

    Ok(())
}