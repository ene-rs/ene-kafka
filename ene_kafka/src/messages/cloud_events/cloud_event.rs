use std::collections::HashMap;

use crate::messages::kafka_message::{Headers, KafkaMessage, ToBytes};

pub trait CloudEvent<Key: ToBytes, Payload: ToBytes>:
    KafkaMessage<Key, Payload> + Sync + Send
{
    fn spec_version(&self) -> anyhow::Result<String>;
    fn event_type(&self) -> anyhow::Result<String>;
    fn event_source(&self) -> anyhow::Result<String>;
    fn event_id(&self) -> anyhow::Result<String>;
    fn event_time(&self) -> anyhow::Result<String>;
    fn event_content_type(&self) -> anyhow::Result<String>;

    fn entity_event_type() -> anyhow::Result<String>;

    fn cloud_event_headers(&self) -> anyhow::Result<Headers> {
        Ok(HashMap::from([
            (String::from("ce_specversion"), self.spec_version()?),
            (String::from("ce_type"), self.event_type()?),
            (String::from("ce_source"), self.event_source()?),
            (String::from("ce_id"), self.event_id()?),
            (String::from("ce_time"), self.event_time()?),
            (String::from("content_type"), self.event_content_type()?),
        ]))
    }
}

pub trait DeserializeFrom<Key: ToBytes, Payload: ToBytes, InputEvent: CloudEvent<Key, Payload>> {
    fn deserialize_from(event: &InputEvent) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub type EventType = String;
