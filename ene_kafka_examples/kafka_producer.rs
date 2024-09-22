use serde::{Deserialize, Serialize};

use ene_kafka::producers::producer::KafkaProducerInterface;
use ene_kafka::{kafka_producer, producers::producer::KafkaProducer};
use ene_kafka_derive::{CloudEvent, DeserializeFrom, KafkaMessage};

#[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
#[kafka(topic = "test", serde = Json, key = entity_id, headers = CloudEvent)]
#[cloud_event(
    content_type = "application/json",
    version = "1.0",
    event_type = "com.ene.entity.updated.v1",
    event_source = "https://ene-kafka.com/docs/cloudevents/entity/updated",
    id = entity_id
)]
struct EntityUpdated {
    pub entity_id: i64,
    pub organisation_id: i64,
}

#[tokio::main]
async fn main() -> ene_kafka::KafkaResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let bootstrap_servers = "localhost:9092".to_string();

    let producer: KafkaProducer = kafka_producer!(bootstrap_servers = bootstrap_servers.clone());
    let event = EntityUpdated {
        entity_id: 1755,
        organisation_id: 42,
    };

    producer.send(event).await?;
    Ok(())
}
