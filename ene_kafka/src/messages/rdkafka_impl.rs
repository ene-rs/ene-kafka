use super::{
    cloud_events::cloud_event::CloudEvent,
    kafka_message::{ContentType, Headers as KafkaHeaders, KafkaTopic},
};
use crate::messages::kafka_message::KafkaMessage;
use anyhow::anyhow;
use rdkafka::{
    message::{BorrowedHeaders, BorrowedMessage, Header, Headers, OwnedHeaders, OwnedMessage},
    Message,
};

pub trait ToRdkafkaHeaders {
    fn to_rdkafka_headers(&self) -> anyhow::Result<rdkafka::message::OwnedHeaders>;
}

impl<'a> KafkaMessage<String, String> for BorrowedMessage<'a> {
    fn topic(&self) -> anyhow::Result<KafkaTopic> {
        Ok(KafkaTopic {
            name: Message::topic(self).to_string(),
            content_type: ContentType::Json,
        })
    }

    fn payload(&self) -> anyhow::Result<String> {
        Message::payload(self)
            .map(|bytes| -> anyhow::Result<String> { Ok(String::from_utf8(bytes.to_vec())?) })
            .ok_or(anyhow!("Payload is not a valid UTF-8 string"))?
    }

    fn key(&self) -> anyhow::Result<String> {
        Message::key(self)
            .map(|bytes| -> anyhow::Result<String> { Ok(String::from_utf8(bytes.to_vec())?) })
            .ok_or(anyhow!("Key is not a valid UTF-8 string"))?
    }

    fn headers(&self) -> anyhow::Result<crate::messages::kafka_message::Headers> {
        Message::headers(self)
            .map(borrowed_headers_to_headers)
            .ok_or(anyhow!("Headers are not a valid UTF-8 string"))?
    }
}

impl KafkaMessage<String, String> for OwnedMessage {
    fn topic(&self) -> anyhow::Result<KafkaTopic> {
        Ok(KafkaTopic {
            name: Message::topic(self).to_string(),
            content_type: ContentType::Json,
        })
    }

    fn payload(&self) -> anyhow::Result<String> {
        Message::payload(self)
            .map(|bytes| -> anyhow::Result<String> { Ok(String::from_utf8(bytes.to_vec())?) })
            .ok_or(anyhow!("Payload is not a valid UTF-8 string"))?
    }

    fn key(&self) -> anyhow::Result<String> {
        Message::key(self)
            .map(|bytes| -> anyhow::Result<String> { Ok(String::from_utf8(bytes.to_vec())?) })
            .ok_or(anyhow!("Key is not a valid UTF-8 string"))?
    }

    fn headers(&self) -> anyhow::Result<crate::messages::kafka_message::Headers> {
        Message::headers(self)
            .map(owned_headers_to_headers)
            .ok_or(anyhow!("Headers are not a valid UTF-8 string"))?
    }
}

impl<'a> CloudEvent<String, String> for BorrowedMessage<'a> {
    fn spec_version(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("ce_specversion")
            .map(|value| value.clone())
            .ok_or(anyhow!("ce_specversion header is missing"))
    }

    fn event_type(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("ce_type")
            .map(|value| value.clone())
            .ok_or(anyhow!("ce_type header is missing"))
    }

    fn event_source(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("ce_source")
            .map(|value| value.clone())
            .ok_or(anyhow!("ce_source header is missing"))
    }

    fn event_id(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("ce_id")
            .map(|value| value.clone())
            .ok_or(anyhow!("ce_id header is missing"))
    }

    fn event_time(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("ce_time")
            .map(|value| value.clone())
            .ok_or(anyhow!("ce_time header is missing"))
    }

    fn event_content_type(&self) -> anyhow::Result<String> {
        KafkaMessage::headers(self)?
            .get("content_type")
            .map(|value| value.clone())
            .ok_or(anyhow!("content_type header is missing"))
    }

    fn entity_event_type() -> anyhow::Result<String> {
        Ok(String::from("lib.rdkafka.BorrowedMessage"))
    }
}

impl ToRdkafkaHeaders for KafkaHeaders {
    fn to_rdkafka_headers(&self) -> anyhow::Result<rdkafka::message::OwnedHeaders> {
        let mut owned_headers = rdkafka::message::OwnedHeaders::new();
        for (key, value) in self.iter() {
            let header = Header {
                key: key.as_str(),
                value: Some(value.as_str()),
            };
            owned_headers = owned_headers.insert(header);
        }
        Ok(owned_headers)
    }
}

pub fn borrowed_headers_to_headers(headers: &BorrowedHeaders) -> anyhow::Result<KafkaHeaders> {
    headers
        .iter()
        .filter(|header| header.value.is_some())
        .map(|header| -> anyhow::Result<(String, String)> {
            let key = header.key.to_string();
            let value = String::from_utf8(
                header
                    .value
                    .ok_or(anyhow!("header with key {key} not avaiable"))?
                    .to_vec(),
            )?;
            Ok((key, value))
        })
        .collect::<anyhow::Result<KafkaHeaders>>()
}

pub fn owned_headers_to_headers(headers: &OwnedHeaders) -> anyhow::Result<KafkaHeaders> {
    headers
        .iter()
        .filter(|header| header.value.is_some())
        .map(|header| -> anyhow::Result<(String, String)> {
            let key = header.key.to_string();
            let value = String::from_utf8(
                header
                    .value
                    .ok_or(anyhow!("header with key {key} not avaiable"))?
                    .to_vec(),
            )?;
            Ok((key, value))
        })
        .collect::<anyhow::Result<KafkaHeaders>>()
}
