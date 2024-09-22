use rdkafka::client::DefaultClientContext;

pub mod consumers;
pub mod dispatchers;
pub mod handlers;
pub mod messages;
pub mod producers;
pub mod admins;

pub type KafkaResult<T> = anyhow::Result<T>;


#[cfg(feature = "rdkafka")]
pub type ConsumerImpl = rdkafka::consumer::StreamConsumer;
#[cfg(feature = "rdkafka")]
pub type ProducerImpl = rdkafka::producer::FutureProducer;
#[cfg(feature = "rdkafka")]
pub type AdminImpl = rdkafka::admin::AdminClient<DefaultClientContext>;