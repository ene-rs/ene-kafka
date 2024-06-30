use async_trait::async_trait;

use crate::dispatchers::EventDispatcher;
use crate::messages::kafka_message::KafkaTopic;
use crate::producers::producer::{KafkaProducer, KafkaProducerInterface};

#[async_trait]
pub trait KafkaConsumerInterface<Dispatcher: EventDispatcher> {
    fn new(consumer_group_id: String, bootstrap_servers: String) -> Self;
    async fn start<'a>(
        &'a self,
        dispatcher: &'a Dispatcher,
        dlq_producer: &'a KafkaProducer,
        topic: KafkaTopic,
        dlq_topic: KafkaTopic,
    );
}

pub struct KafkaConsumer<
    Dispatcher: EventDispatcher,
    InnerConsumer: KafkaConsumerInterface<Dispatcher> = rdkafka::consumer::StreamConsumer, // Currently, the inner consumer implementation defaults to rdKafka
> {
    topic: KafkaTopic,
    dlq_topic: KafkaTopic,
    dispatcher: Dispatcher,
    inner_consumer: InnerConsumer,
    dlq_producer: KafkaProducer,
}

impl<Dispatcher: EventDispatcher, Consumer: KafkaConsumerInterface<Dispatcher>>
    KafkaConsumer<Dispatcher, Consumer>
{
    pub fn new(
        topic: KafkaTopic,
        dlq_topic: KafkaTopic,
        consumer_group_id: String,
        bootstrap_servers: String,
        handler: Dispatcher,
    ) -> Self {
        let dlq_producer = <KafkaProducer>::new(bootstrap_servers.clone());
        Self {
            topic,
            dlq_topic,
            dispatcher: handler,
            inner_consumer: Consumer::new(consumer_group_id, bootstrap_servers),
            dlq_producer,
        }
    }

    pub async fn start(self) {
        self.inner_consumer
            .start(
                &self.dispatcher,
                &self.dlq_producer,
                self.topic,
                self.dlq_topic,
            )
            .await;
    }
}

///
/// Create a new Kafka consumer
/// Arguments:
/// - `topic` - a string representing the Kafka topic
/// - `dlq_topic` - a string representing the Kafka dead letter queue topic. If an event could not be consuler, it will be sent to the dead letter queue.
/// - `consumer_group_id` - a string representing the Kafka consumer group id
/// - `bootstrap_servers` - a string representing the Kafka bootstrap servers
/// - `handlers` - a map of handlers and their types to be used by the consumer.
/// The handlers need to implement The `EventHandler` trait.
/// 
/// Example:
/// ```rust,ignore
/// let consumer = kafka_consumer!(
///    topic = "test".to_string(),
///    dlq_topic = "test-dlq".to_string(),
///    consumer_group_id = "test-consumer".to_string(),
///    bootstrap_servers = "localhost:9092".to_string(),
///    handlers = [EntitiyCreated -> entity_created_handler, EntityUpdated -> entity_updated_handler],
/// );
/// consumer.start().await;
/// ```
/// 
#[macro_export]
macro_rules! kafka_consumer {
    (
        topic = $topic: expr,
        dlq_topic = $dlq_topic: expr,
        consumer_group_id = $consumer_group_id: expr,
        bootstrap_servers = $bootstrap_servers: expr,
        handlers = [$($handler_type: ident -> $handler: expr),*]$(,)?
        $(,)?
    ) => {
        {

            ene_kafka::generate_event_dispatcher!($($handler_type),*);


            ene_kafka::consumers::consumer::KafkaConsumer::<CloudEventDispatcher>::new(
                $topic,
                $dlq_topic,
                $consumer_group_id.to_string(),
                $bootstrap_servers.to_string(),
                CloudEventDispatcher { handlers: ene_kafka::create_handlers!($($handler),*)},
            )

        }

    };
}
