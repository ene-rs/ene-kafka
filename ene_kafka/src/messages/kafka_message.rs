use std::collections::HashMap;

use anyhow::Result;

pub type HeaderKey = String;
pub type HeaderValue = String;
pub type Headers = HashMap<HeaderKey, HeaderValue>;

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl ToBytes for String {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
}

impl ContentType {
    pub fn from_str(content_type: &str) -> Result<Self> {
        match content_type {
            "json" => Ok(Self::Json),
            _ => Err(anyhow::anyhow!("Invalid content type")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KafkaTopic {
    pub name: String,
    pub content_type: ContentType,
}

pub trait KafkaMessage<Key: ToBytes, Payload: ToBytes>: Sync + Send {
    fn topic(&self) -> anyhow::Result<KafkaTopic>;
    fn payload(&self) -> anyhow::Result<Payload>;
    fn key(&self) -> anyhow::Result<Key>;
    fn headers(&self) -> anyhow::Result<Headers>;
}
