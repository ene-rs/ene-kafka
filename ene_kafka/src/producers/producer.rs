extern crate proc_macro;

use async_trait::async_trait;

use crate::messages::kafka_message::{KafkaMessage, ToBytes};

#[async_trait]
pub trait KafkaProducerInterface: Sync + Send {
    async fn send<Key: ToBytes, Payload: ToBytes, Message: KafkaMessage<Key, Payload>>(
        &self,
        message: Message,
    ) -> anyhow::Result<()>
where;
    fn new(bootstrap_servers: String) -> Self;
}

#[derive(Clone)]
pub struct KafkaProducer<Producer: KafkaProducerInterface = rdkafka::producer::FutureProducer> {
    producer: Producer,
}

#[async_trait]
impl<A: KafkaProducerInterface> KafkaProducerInterface for KafkaProducer<A> {
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
