use async_trait::async_trait;

use crate::messages::cloud_events::cloud_event::CloudEvent;

#[async_trait]
pub trait EventDispatcher: Send + Sync {
    async fn dispatch_event<Event: CloudEvent<String, String>>(
        &self,
        event: &Event,
    ) -> anyhow::Result<()>;
}

#[macro_export]
macro_rules! generate_handler_types_enum {
    ($($handler: ident $(< $( $identifier:tt $( : $identifier_constraint:tt $(+ $identifier_additions:tt )* )? ),+ >)?),*) => {
        enum HandlerTypes {
            $($handler($handler $(< $( $generic_identifier $( : $identifier_constraint $(+ $identifier_additions )* )? ),+ >)?)),*
        }

        $(
            impl From<$handler $(< $( $generic_identifier $( : $identifier_constraint $(+ $identifier_additions )* )? ),+ >)?> for HandlerTypes {
                fn from(handler: $handler $(< $( $generic_identifier $( : $identifier_constraint $(+ $identifier_additions )* )? ),+ >)?) -> Self {
                    HandlerTypes::$handler(handler)
                }
            }
        )*
    }
}

/// A macro to generate an event dispatcher struct that will dispatch events to the appropriate handlers
/// based on the event type.
/// The macro expects a list of handlers that will be used to dispatch the events.
#[macro_export]
macro_rules! generate_event_dispatcher {
    ($($handler: ident $(< $( $generic_identifier:tt $( : $identifier_constraint:tt $(+ $identifier_additions:tt )* )? ),+ >)?),*) => {
        ene_kafka::generate_handler_types_enum!($($handler $(< $( $generic_identifier $( : $identifier_constraint $(+ $identifier_additions )* )? ),+ >)?),*);
        struct CloudEventDispatcher {
            handlers: Vec<HandlerTypes>,
        }


    #[async_trait::async_trait]
    impl ene_kafka::dispatchers::EventDispatcher for CloudEventDispatcher {

        async fn dispatch_event<Event: ene_kafka::messages::cloud_events::cloud_event::CloudEvent<String, String>>(&self, event: &Event) -> anyhow::Result<()> {
            for handler in self.handlers.iter() {
                match handler {
                    $(HandlerTypes::$handler(handler) => {
                        if ene_kafka::handlers::EventHandler::can_handle(handler, event)? {
                            ene_kafka::handlers::EventHandler::deserialize_and_handle(handler, event).await?;
                        }
                    }),*
                }
            }
            Ok(())
        }
    }
    }
}
