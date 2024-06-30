use serde::{Deserialize, Serialize};

use ene_kafka::messages::cloud_events::cloud_event::{CloudEvent, DeserializeFrom};
use ene_kafka::producers::producer::KafkaProducerInterface;
use ene_kafka::{kafka_producer, producers::producer::KafkaProducer};
use ene_kafka_derive::{CloudEvent, KafkaMessage};

#[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize)]
#[kafka(topic = "test", serde = Json, key = entity_id, headers = CloudEvent)]
#[cloud_event(
    content_type = "application/json",
    version = "1.0",
    event_type = "com.ene.entity.created.v1",
    event_source = "https://ene-kafka.com/docs/cloudevents/entity/created"
)]
struct EntityCreated {
    pub entity_id: i64,
    pub organisation_id: i64,
}

impl<Event: CloudEvent<String, String>> DeserializeFrom<String, String, Event> for EntityCreated {
    fn deserialize_from(value: &Event) -> ene_kafka::KafkaResult<Self> {
        Ok(serde_json::from_str::<EntityCreated>(
            value.payload()?.as_str(),
        )?)
    }
}

#[tokio::main]
async fn main() -> ene_kafka::KafkaResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let bootstrap_servers = "localhost:9092".to_string();

    let producer: KafkaProducer = kafka_producer!(bootstrap_servers = bootstrap_servers.clone());
    let event = EntityCreated {
        entity_id: 1755,
        organisation_id: 42,
    };

    producer.send(event).await?;
    Ok(())
}
