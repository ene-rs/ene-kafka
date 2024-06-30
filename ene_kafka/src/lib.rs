pub mod consumers;
pub mod dispatchers;
pub mod handlers;
pub mod messages;
pub mod producers;
pub mod admins;

pub type KafkaResult<T> = anyhow::Result<T>;
