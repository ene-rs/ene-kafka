use async_trait::async_trait;

use crate::messages::cloud_events::cloud_event::CloudEvent;

#[async_trait]
pub trait EventDispatcher: Send + Sync {
    async fn dispatch_event<Event: CloudEvent<String, String>>(
        &self,
        event: &Event,
    ) -> anyhow::Result<()>;
}

/// A macro to generate an event dispatcher struct that will dispatch events to the appropriate handlers
/// based on the event type.
/// The macro expects a list of handlers that will be used to dispatch the events.
#[macro_export]
macro_rules! generate_event_dispatcher {
    ($($handler_name: ident: $handler_type: ident $(< $( $generic_identifier:tt $( : $identifier_constraint:tt $(+ $identifier_additions:tt )* )? ),+ >)?),*) => {
        struct CloudEventDispatcher {
           $(
               $handler_name: $handler_type $(< $( $generic_identifier $( : $identifier_constraint $(+ $identifier_additions )* )? ),+ >)?,
           )*
        }


    #[async_trait::async_trait]
    impl ene_kafka::dispatchers::EventDispatcher for CloudEventDispatcher {

        async fn dispatch_event<Event: ene_kafka::messages::cloud_events::cloud_event::CloudEvent<String, String>>(&self, event: &Event) -> anyhow::Result<()> {
            use ene_kafka::handlers::EventHandler;
            $(
                if self.$handler_name.can_handle(event)? {
                    return self.$handler_name.deserialize_and_handle(event).await;
                }
            )*
            anyhow::bail!("No handler found for event type {:?}", event.event_type()?);
        }
    }
    }
}
