extern crate proc_macro;

use async_trait::async_trait;

use crate::{
    messages::kafka_message::{KafkaMessage, ToBytes},
    ProducerImpl,
};

#[async_trait]
pub trait KafkaProducerInterface: Sync + Send {
    async fn send<Key: ToBytes, Payload: ToBytes, Message: KafkaMessage<Key, Payload>>(
        &self,
        message: Message,
    ) -> anyhow::Result<()>
where;
    fn new(bootstrap_servers: String) -> Self;
}

#[derive(Debug, Clone)]
pub struct KafkaProducer<Producer: KafkaProducerInterface = ProducerImpl> {
    producer: Producer,
}

#[async_trait]
impl<A: KafkaProducerInterface> KafkaProducerInterface for KafkaProducer<A> {
    ///
    /// Sends a message to a kafka topic
    /// Arguments:
    /// - `message` - a KafkaMessage
    ///
    /// Example:
    /// ```rust, ignore
    /// #[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
    /// #[kafka(topic = "test", serde = Json, key = entity_id, headers = CloudEvent)]
    /// #[cloud_event(
    /// content_type = "application/json",
    /// version = "1.0",
    /// event_type = "com.ene.entity.created.v1",
    /// event_source = "https://ene-kafka.com/docs/cloudevents/entity/created"
    /// )]
    /// struct EntityCreated {
    ///     pub entity_id: i64,
    ///     pub organisation_id: i64,
    /// }
    ///
    /// let producer = kafka_producer!(bootstrap_servers = "localhost:9092".to_string());
    /// let event = EntityCreated {
    ///     entity_id: 1,
    ///     organisation_id: 1,
    /// };
    /// producer.send(event).await?;
    async fn send<Key: ToBytes, Payload: ToBytes, Message: KafkaMessage<Key, Payload>>(
        &self,
        message: Message,
    ) -> anyhow::Result<()> {
        tracing::debug!("sending message");
        self.producer.send(message).await
    }

    fn new(bootstrap_servers: String) -> Self {
        Self {
            producer: A::new(bootstrap_servers),
        }
    }
}

///
/// Create a new Kafka producer
/// Arguments:
/// - `bootstrap_servers` - a string representing the Kafka bootstrap servers
///
/// Example:
/// ```rust, ignore
/// let producer = kafka_producer!(bootstrap_servers = "localhost:9092".to_string());
/// producer.send(event).await?;
/// ```
///
#[macro_export]
macro_rules! kafka_producer {
    (bootstrap_servers = $bootstrap_servers: expr) => {
        <ene_kafka::producers::producer::KafkaProducer>::new($bootstrap_servers)
    };
}
