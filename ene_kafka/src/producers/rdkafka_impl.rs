use async_trait::async_trait;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};

use crate::messages::{
    kafka_message::{KafkaMessage, ToBytes},
    rdkafka_impl::ToRdkafkaHeaders,
};

use super::producer::KafkaProducerInterface;

#[async_trait]
impl KafkaProducerInterface for FutureProducer {
    async fn send<Key: ToBytes, Payload: ToBytes, Message: KafkaMessage<Key, Payload>>(
        &self,
        message: Message,
    ) -> anyhow::Result<()> {
        let payload = message.payload()?.to_bytes()?;
        let key = message.key()?.to_bytes()?;
        let topic = message.topic()?;
        let record: FutureRecord<'_, Vec<u8>, Vec<u8>> = FutureRecord::<Vec<u8>, Vec<u8>> {
            topic: topic.name.as_str(),
            partition: None,
            payload: Some(&payload),
            key: Some(&key),
            timestamp: None,
            headers: Some(message.headers()?.to_rdkafka_headers()?),
        };
        let delivery_status = FutureProducer::send(self, record, Timeout::Never).await;
        match delivery_status {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!(format!(
                "Failed to produce event: {:?}",
                e
            ))),
        }
    }

    fn new(bootstrap_servers: String) -> Self {
        // TODO: configure
        ClientConfig::new()
            .set("request.required.acks", "all")
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("producers::rdkafka_impl - failed to create producer")
    }
}
