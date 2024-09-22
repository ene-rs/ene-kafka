use async_trait::async_trait;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::ClientConfig;

use crate::dispatchers::EventDispatcher;
use crate::messages::kafka_message::KafkaTopic;
use crate::producers::producer::{KafkaProducer, KafkaProducerInterface};

use super::consumer::KafkaConsumerInterface;

#[async_trait]
impl<Dispatcher: EventDispatcher, InnerProducer: KafkaProducerInterface>
    KafkaConsumerInterface<Dispatcher, InnerProducer> for StreamConsumer
{
    fn new(consumer_group_id: String, bootstrap_servers: String) -> Self {
        tracing::info!("Creating consumer with group ID {}", consumer_group_id);
        // TODO: configure all properties
        ClientConfig::new()
            .set("group.id", consumer_group_id)
            .set("bootstrap.servers", bootstrap_servers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create::<StreamConsumer>()
            .expect("Consumer creation failed")
    }

    async fn start<'a>(
        &'a self,
        dispatcher: &'a Dispatcher,
        dlq_producer: &'a KafkaProducer<InnerProducer>,
        topic: KafkaTopic,
        dlq_topic: KafkaTopic,
    ) {
        self.subscribe(&[topic.name.as_str()])
            .map(|()| tracing::info!("Subscribed to {}", topic.name.as_str()))
            .expect("Can't subscribe to specified topics");
        loop {
            match self.recv().await {
                Ok(event) => {
                    tracing::debug!("event: {:?}", event);
                    let result = &dispatcher.dispatch_event(&event).await;
                    match result {
                        Ok(_) => {}
                        Err(error) => {
                            tracing::error!("consumers::rdkafka_impl::error: {:?}", error);
                            let unhandled_event = event.detach().set_topic(dlq_topic.name.clone());
                            match dlq_producer.send(unhandled_event).await {
                                Ok(_) => {
                                    tracing::info!("Sent event to DLQ");
                                }
                                Err(error) => {
                                    tracing::error!(
                                        "consumers::rdkafka_impl::dlq::error: {:?}",
                                        error
                                    );
                                }
                            }
                        }
                    }
                    match self.commit_message(&event, CommitMode::Async) {
                        Ok(_) => {}
                        Err(error) => {
                            tracing::error!(
                                "consumers::rdkafka_impl::commit_message::error: {:?}",
                                error
                            );
                        }
                    }
                }
                Err(error) => {
                    tracing::error!("Kafka error: {}", error);
                }
            }
        }
    }
}
