use async_trait::async_trait;

use crate::dispatchers::EventDispatcher;
use crate::messages::kafka_message::KafkaTopic;
use crate::producers::producer::{KafkaProducer, KafkaProducerInterface};
use crate::{ConsumerImpl, ProducerImpl};

#[async_trait]
pub trait KafkaConsumerInterface<Dispatcher: EventDispatcher, InnerProducer: KafkaProducerInterface> {
    fn new(consumer_group_id: String, bootstrap_servers: String) -> Self;
    async fn start<'a>(
        &'a self,
        dispatcher: &'a Dispatcher,
        dlq_producer: &'a KafkaProducer<InnerProducer>,
        topic: KafkaTopic,
        dlq_topic: KafkaTopic,
    );
}


#[derive(Debug, Clone)]
pub struct KafkaConsumer<
    Dispatcher: EventDispatcher,
    InnerConsumer: KafkaConsumerInterface<Dispatcher, InnerProducer> = ConsumerImpl,
    InnerProducer: KafkaProducerInterface = ProducerImpl,
> {
    topic: KafkaTopic,
    dlq_topic: KafkaTopic,
    dispatcher: Dispatcher,
    inner_consumer: InnerConsumer,
    dlq_producer: KafkaProducer<InnerProducer>,
}

impl<Dispatcher: EventDispatcher, Consumer: KafkaConsumerInterface<Dispatcher, InnerProducer>, InnerProducer: KafkaProducerInterface>
    KafkaConsumer<Dispatcher, Consumer, InnerProducer>
{
    pub fn new(
        topic: KafkaTopic,
        dlq_topic: KafkaTopic,
        consumer_group_id: String,
        bootstrap_servers: String,
        handler: Dispatcher,
    ) -> Self {
        let dlq_producer = KafkaProducer::new(bootstrap_servers.clone());
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
///    let consumer = kafka_consumer!(
///        topic = KafkaTopic {
///            name: "test".to_string(),
///            content_type: ContentType::Json
///        },
///        dlq_topic = KafkaTopic {
///            name: "test-dlq".to_string(),
///            content_type: ContentType::Json
///        },
///        consumer_group_id = "test-group",
///        bootstrap_servers = bootstrap_servers,
///        handlers = {
///            entity_created_event_handler: EntityCreatedEventHandler = EntityCreatedEventHandler {}
///        }
///    );
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
        handlers = {$($handler_name: ident: $handler_type: ident = $handler: expr),*}$(,)?
        $(,)?
    ) => {
        {

            ene_kafka::generate_event_dispatcher!($($handler_name: $handler_type),*);


            ene_kafka::consumers::consumer::KafkaConsumer::<CloudEventDispatcher>::new(
                $topic,
                $dlq_topic,
                $consumer_group_id.to_string(),
                $bootstrap_servers.to_string(),
                CloudEventDispatcher { $($handler_name: $handler),* }
            )

        }

    };
}

