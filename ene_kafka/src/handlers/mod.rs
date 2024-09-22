use async_trait::async_trait;

use crate::messages::cloud_events::cloud_event::{CloudEvent, DeserializeFrom, EventType};


#[async_trait]
pub trait EventHandler<
    InputEvent: CloudEvent<String, String>,
    HandlableEvent: CloudEvent<String, String> + DeserializeFrom<String, String, InputEvent>,
>
{
    fn can_handle(&self, event: &InputEvent) -> anyhow::Result<bool> {
        Ok(event.event_type()? == self.event_type()?)
    }

    fn event_type(&self) -> anyhow::Result<EventType>;

    async fn deserialize_and_handle(&self, event: &InputEvent) -> anyhow::Result<()> {
        let deserialized_event = HandlableEvent::deserialize_from(&event)?;
        self.handle(&deserialized_event).await
    }

    async fn handle(&self, event: &HandlableEvent) -> anyhow::Result<()>;
}