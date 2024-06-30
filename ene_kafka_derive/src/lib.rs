mod cloud_event;
mod deserialize_from;
mod handler;
mod kafka_message;

/// Derive the KafkaMessage trait for a struct
/// It requires the following attributes:
/// - `key` - the name of the field that will be used as the key
/// - `topic` - the name of the field that will be used as the topic
/// - `headers` - the name of the field that will be used as the headers. Possible values: `CloudEvent` or `None` (default)
/// - `payload` - the name of the field that will be used as the payload
/// - `serde` - the serialization format of the payload. Possible values: `Json`
/// 
/// Example:
/// ```rust,ignore
/// #[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize)]
/// #[kafka(topic = "test", serde = Json, key = message_id)]
/// struct SomeMessage {
///    pub message_id: i64,
/// }
/// ```
#[proc_macro_derive(KafkaMessage, attributes(kafka))]
pub fn kafkamessage_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    kafka_message::kafkamessage_derive_macro2(input.into())
        .unwrap()
        .into()
}

/// Derive the CloudEvent trait for a struct
/// It requires the following attributes:
/// - `content_type` - the content type of the event
/// - `version` - the version of the event
/// - `event_type` - the type of the event
/// - `event_source` - the source of the event
/// 
/// Implementing `KafkaMessage` is required for this trait to work. The `headers` field of the `KafkaMessage` trait should be set to `CloudEvent`
/// 
/// Example:
/// ```rust, ignore
/// #[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize)]
/// #[kafka(topic = "test", serde = Json, key = event_id, headers = CloudEvent)]
/// #[cloud_event(
///    content_type = "application/json",
///    version = "1.0",
///    event_type = "com.ene.SomeEvent.v1",
///    event_source = "https://ene-kafka.com/docs/cloudevents/SomeEvent"
/// )]
/// struct SomeEvent {
///   pub event_id: i64,
/// }
/// ```
#[proc_macro_derive(CloudEvent, attributes(cloud_event))]
pub fn cloudevent_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cloud_event::cloudevent_derive_macro2(input.into())
        .unwrap()
        .into()
}

/// Derive the EventHandler trait for a struct
/// It requires the following attributes:
/// - `event` - A concrete type that implements the `CloudEvent` trait
/// - `handler` - The name of the handler function. This function should be implemented by the struct. It should take a reference to the event it can handle as input.
/// 
/// The event type should implement `CloudEvent` as well as `DeserializeFrom` is required for this trait to work.
/// Example:
/// ```rust,ignore
/// #[derive(EventHandler)]
/// #[event_handler(event = crate::SomeEvent, handler = handle_some_event)]
/// struct SomeEventHandler;
///
/// impl SomeEventHandler {
///   async fn handle_some_event(&self, event: &crate::SomeEvent) -> anyhow::Result<()> {
///    println!("Handling event: {:?}", event);
///    Ok(())
///  }
/// }
/// ```
#[proc_macro_derive(EventHandler, attributes(event_handler))]
pub fn handler_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    handler::handler_derive_macro2(input.into()).unwrap().into()
}

/// Derive the DeserializeFrom trait for a struct
/// It relies on the `KafkaMessage` trait and requires the following attributes:
/// - `serde` - the serialization format of the payload. Possible values: `Json`
/// `DeserializeFrom` requires the struct to implement `Deserialize` from the `serde` crate.
/// 
/// Example:
/// ```rust, ignore
/// #[derive(KafkaMessage, Serialize, CloudEvent, Debug, Deserialize, DeserializeFrom)]
/// #[kafka(topic = "test", serde = Json, key = message_id)]
/// struct SomeMessage {
///   pub message_id: i64,
/// }
/// ```
#[proc_macro_derive(DeserializeFrom, attributes(kafka))]
pub fn deserialize_from_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    deserialize_from::deserialize_from_derive_macro2(input.into())
        .unwrap()
        .into()
}
