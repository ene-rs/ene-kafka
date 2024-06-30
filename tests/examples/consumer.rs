use serde::{Deserialize, Serialize};

use ene_kafka::handlers::EventHandler;
use ene_kafka::kafka_consumer;
use ene_kafka::messages::kafka_message::{ContentType, KafkaTopic};
use ene_kafka_derive::{CloudEvent, DeserializeFrom, EventHandler, KafkaMessage};

#[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
#[kafka(topic = "test", serde = Json, key = entity_id, headers = CloudEvent)]
#[cloud_event(
    content_type = "application/json",
    version = "1.0",
    event_type = "com.ene.entity.created.v1",
    event_source = "https://ene.com/docs/cloudevents/entity/created"
)]
struct EntityCreated {
    pub entity_id: i64,
    pub organisation_id: i64,
}

#[tokio::main]
async fn main() -> ene_kafka::KafkaResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let bootstrap_servers = "localhost:9092".to_string();

    let consumer = kafka_consumer!(
        topic = KafkaTopic {
            name: "test".to_string(),
            content_type: ContentType::Json
        },
        dlq_topic = KafkaTopic {
            name: "test-dlq".to_string(),
            content_type: ContentType::Json
        },
        consumer_group_id = "test-group",
        bootstrap_servers = bootstrap_servers,
        handlers = [
            EntityCreatedEventHandler -> EntityCreatedEventHandler {}
        ]
    );
    tokio::spawn(async move { consumer.start().await });

    loop {}
}

#[derive(EventHandler)]
#[event_handler(event = EntityCreated, handler = handle_entity_created_event)]
struct EntityCreatedEventHandler {}

impl EntityCreatedEventHandler {
    async fn handle_entity_created_event(&self, event: &EntityCreated) -> ene_kafka::KafkaResult<()> {
        println!("EntityCreatedEventHandler: {:?}", event);
        Ok(())
    }
}
