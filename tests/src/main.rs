use std::env;

use serde::{Deserialize, Serialize};

use ene_kafka::handlers::EventHandler;
use ene_kafka::messages::kafka_message::{ContentType, KafkaTopic};
use ene_kafka::producers::producer::KafkaProducerInterface;
use ene_kafka::admins::KafkaAdminInterface;
use ene_kafka::{kafka_consumer, kafka_producer, producers::producer::KafkaProducer};
use ene_kafka_derive::{CloudEvent, DeserializeFrom, EventHandler, KafkaMessage};

#[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
#[kafka(
    topic           = "test", 
    serde           = Json,
    key             = entity_id,
    headers         = CloudEvent)]
#[cloud_event(
    content_type = "application/json",
    version = "1.0",
    event_type = "com.ene.en tity.created.v1",
    event_source = "https://ene.com/docs/cloudevents/entity/created"
)]
struct EntityCreated {
    pub entity_id: i64,
    pub organisation_id: i64,
}

#[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
#[kafka(
    topic           = "test", 
    serde           = Json,
    key             = entity_id,
    headers         = CloudEvent)]
#[cloud_event(
    content_type = "application/json",
    version = "1.0",
    event_type = "com.ene.entity.created.v1",
    event_source = "https://ene.com/docs/cloudevents/entity/created"
)]
struct EntityCreatedWrong {
    pub entity_id: i64,
    pub org_id: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");

    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

    let topic = "test".to_string();
    let bootstrap_servers = "localhost:9092".to_string();
    let kafka_admin = ene_kafka::kafka_admin!(bootstrap_servers = bootstrap_servers.clone(), request_time_out_ms = "50000".to_string(), connection_max_idle_ms = "0".to_string());

    kafka_admin.create_topic_if_not_exists(&topic, 20, 1).await?;
    
    println!("topic {topic} is alive: {}", kafka_admin.check_topic_liveness(&topic).await?);

    let producer: KafkaProducer = kafka_producer!(bootstrap_servers = bootstrap_servers.clone());
    let event = EntityCreated {
        entity_id: 1755,
        organisation_id: 42,
    };

    let entity_created_event_handler: EntityCreatedEventHandler = EntityCreatedEventHandler {
        _effy_service: "effy".to_string(),
    };

    let consumer = kafka_consumer!(
        topic = KafkaTopic {
            name: topic.clone(),
            content_type: ContentType::Json
        },
        dlq_topic = KafkaTopic {
            name: "test-dlq".to_string(),
            content_type: ContentType::Json
        },
        consumer_group_id = "test-group",
        bootstrap_servers = bootstrap_servers,
        handlers = [
            EntityCreatedEventHandler -> entity_created_event_handler
        ]
    );
    tokio::spawn(async move { consumer.start().await });
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    producer.send(event).await?;
    loop {}
}

#[derive(EventHandler)]
#[event_handler(event = crate::EntityCreated, handler = handle)]
struct EntityCreatedEventHandler {
    pub _effy_service: String,
}

impl EntityCreatedEventHandler {
    async fn handle(&self, event: &EntityCreated) -> anyhow::Result<()> {
        println!("entity_created: {:?}", event);
        Ok(())
    }
}
