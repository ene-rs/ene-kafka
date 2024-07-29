use anyhow::anyhow;
use async_trait::async_trait;
use rdkafka::{admin::{AdminClient, AdminOptions, ConfigResource, NewTopic, OwnedResourceSpecifier, ResourceSpecifier}, client::DefaultClientContext, config::FromClientConfig, types::RDKafkaErrorCode};
use tracing::{error, info};

use super::KafkaAdminInterface;

#[async_trait]
impl KafkaAdminInterface for AdminClient<DefaultClientContext> {
    async fn verify_topic_existence(&self, topic: &str) -> anyhow::Result<bool> {
        let topic_config = ResourceSpecifier::Topic(topic);
        let admin_options =
            AdminOptions::new().request_timeout(Some(std::time::Duration::from_secs(1)));
        self.describe_configs(vec![&topic_config], &admin_options)
            .await
            .map(|res: Vec<Result<ConfigResource, RDKafkaErrorCode>>| {
                res.iter().any(|res| match res {
                    Ok(config_resource) => match &config_resource.specifier {
                        OwnedResourceSpecifier::Topic(recieved_topic) => {
                            let topic_exists = (recieved_topic == topic) && !config_resource.entries.is_empty();
                            if topic_exists {
                                info!("Topic {} exists", topic);
                            };
                            topic_exists
                        }
                        something_else => {
                            error!("Expected topic, got {:?}", something_else);
                            false
                        }
                    },
                    Err(e) => {
                        error!("Error describing topic: {:?}", e);
                        false
                    }
                })
            })
            .map_err(|e| {
                error!("Error describing topic: {:?}", e);
                anyhow!(e.to_string())
            })
    }
    async fn create_topic_if_not_exists(&self, topic: &str, partitions: i32, replication_factor: i32) -> anyhow::Result<()> {
        let topic_exists = self.verify_topic_existence(topic).await?;
        if !topic_exists {
            let new_topic = NewTopic::new(
                topic,
                partitions,
                rdkafka::admin::TopicReplication::Fixed(
                    replication_factor,
                ),
            );
            self.create_topics(
                    &[new_topic],
                    &AdminOptions::new()
                        .request_timeout(Some(std::time::Duration::from_secs(1)))
                        .operation_timeout(Some(std::time::Duration::from_secs(1))),
                )
                .await
                .map(|res| {
                    res.iter().all(|res| match res {
                        Ok(_) => true,
                        Err(e) => {
                            error!("Error creating topic: {:?}", e);
                            false
                        }
                    })
                })
                .map_err(|e| {
                    error!("Error creating topic: {:?}", e);
                    anyhow!(e.to_string())
                })?;
        }
        Ok(())
    }
    async fn check_topic_liveness(&self, topic: &str) -> anyhow::Result<bool> {
        let topic_config = ResourceSpecifier::Topic(topic);
        let admin_options = AdminOptions::new();
        self.describe_configs(vec![&topic_config], &admin_options)
            .await
            .map(|res: Vec<Result<ConfigResource, RDKafkaErrorCode>>| {
                res.iter().all(|res| 
                    res.is_ok())
                    && res.iter().any(|res| match res {
                        Ok(config_resource) => match &config_resource.specifier {
                            OwnedResourceSpecifier::Topic(recieved_topic) => {
                                (recieved_topic == topic) && !config_resource.entries.is_empty()
                            }
                            something_else => {
                                error!("Expected topic, got {:?}", something_else);
                                false
                            }
                        },
                        Err(e) => {
                            error!("Error describing topic: {:?}", e);
                            false
                        }
                    })
            })
            .map_err(|e| {
                error!("Error describing topic: {:?}", e);
                anyhow!(e.to_string())
            })
    }

    fn new(bootstrap_servers: String, request_time_out_ms: String, connection_max_idle_ms: String) -> Self {
        AdminClient::from_config(
            &rdkafka::ClientConfig::new()
                .set("bootstrap.servers", bootstrap_servers)
                .set("request.timeout.ms", &request_time_out_ms)
                .set(
                    "connections.max.idle.ms",
                    &connection_max_idle_ms.to_string(),
                ),
        )
        .expect("Failed to create admin client")
    }
    
}