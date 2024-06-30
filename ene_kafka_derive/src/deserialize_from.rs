use syn::DeriveInput;

use crate::kafka_message::KafkaMessageAttributes;

pub fn deserialize_from_derive_macro2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input.into())?;
    let KafkaMessageAttributes { serde, .. }: KafkaMessageAttributes =
        deluxe::extract_attributes(&mut ast)?;
    let struct_name = &ast.ident;

    Ok(quote::quote! {
        impl<Event: ene_kafka::messages::cloud_events::cloud_event::CloudEvent<String, String>> ene_kafka::messages::cloud_events::cloud_event::DeserializeFrom<String, String, Event> for #struct_name {
            fn deserialize_from(value: &Event) -> ene_kafka::KafkaResult<Self> {
                match ene_kafka::messages::kafka_message::ContentType::#serde {
                    ene_kafka::messages::kafka_message::ContentType::Json => {
                        Ok(serde_json::from_str::<#struct_name>(value.payload()?.as_str())?)
                    }
                }
            }
        }
    })
}
