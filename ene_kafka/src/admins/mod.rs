pub mod rdkafka_impl;

use async_trait::async_trait;

use crate::AdminImpl;


#[async_trait]
pub trait KafkaAdminInterface {
    async fn verify_topic_existence(&self, topic: &str) -> anyhow::Result<bool>;
    async fn create_topic_if_not_exists(&self, topic: &str, partitions: i32, replication_factor: i32) -> anyhow::Result<()>;
    async fn check_topic_liveness(&self, topic: &str) -> anyhow::Result<bool>;
    fn new(bootstrap_servers: String, request_time_out_ms: String, connection_max_idle_ms: String)-> Self;
}



#[derive(Debug, Clone)]
pub struct KafkaAdmin<Admin: KafkaAdminInterface = AdminImpl> {
    admin: Admin,
}

#[async_trait]
impl<A: KafkaAdminInterface + std::marker::Sync + std::marker::Send> KafkaAdminInterface for KafkaAdmin<A> {
    async fn verify_topic_existence(&self, topic: &str) -> anyhow::Result<bool> {
        self.admin.verify_topic_existence(topic).await
    }
    async fn create_topic_if_not_exists(&self, topic: &str, partitions: i32, replication_factor: i32) -> anyhow::Result<()> {
        self.admin.create_topic_if_not_exists(topic, partitions, replication_factor).await
    }
    async fn check_topic_liveness(&self, topic: &str) -> anyhow::Result<bool> {
        self.admin.check_topic_liveness(topic).await
    }
    fn new(bootstrap_servers: String, request_time_out_ms: String, connection_max_idle_ms: String) -> Self {
        Self {
            admin: A::new(bootstrap_servers, request_time_out_ms, connection_max_idle_ms),
        }
    }
}


///
/// Create a new Kafka admin
/// Arguments:
/// - `bootstrap_servers` - a string representing the Kafka bootstrap servers
/// - `request_time_out_ms` - a string representing the Kafka request time out in milliseconds
/// - `connection_max_idle_ms` - a string representing the Kafka connection max idle time in milliseconds
/// 
/// Example:
/// ```rust,ignore
/// use ene_kafka::admins::KafkaAdminInterface;
/// use ene_kafka::kafka_admin;
/// let admin = kafka_admin!(bootstrap_servers = "localhost:9092".to_string(), request_time_out_ms = "50000".to_string(), connection_max_idle_ms = "0".to_string());
/// admin.check_topic_liveness("topic").await?;
/// ```
///
#[macro_export]
macro_rules! kafka_admin {
    (bootstrap_servers = $bootstrap_servers: expr, request_time_out_ms = $request_time_out_ms: expr, connection_max_idle_ms = $connection_max_idle_ms: expr) => {
        <ene_kafka::admins::KafkaAdmin>::new($bootstrap_servers, $request_time_out_ms, $connection_max_idle_ms)
    };
}