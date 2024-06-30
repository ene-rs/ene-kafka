use std::fmt::{Display, Formatter};

use syn::DeriveInput;

enum HeaderType {
    CloudEvent,
    Empty,
}

impl From<syn::Ident> for HeaderType {
    fn from(ident: syn::Ident) -> Self {
        match ident.to_string().as_str() {
            "CloudEvent" => HeaderType::CloudEvent,
            _ => HeaderType::Empty,
        }
    }
}

impl Display for HeaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderType::CloudEvent => write!(f, "CloudEvent"),
            HeaderType::Empty => write!(f, "Custom"),
        }
    }
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(kafka))]
pub struct KafkaMessageAttributes {
    pub topic: String,
    pub serde: syn::Ident,
    pub key: syn::Ident,
    #[deluxe(default = syn::Ident::new("Empty", proc_macro2::Span::call_site()))]
    pub headers: syn::Ident,
}

pub fn kafkamessage_derive_macro2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // convert into an ast
    let mut ast: DeriveInput = syn::parse2((input).into())?;

    // extract struct attributes
    let KafkaMessageAttributes {
        topic,
        key,
        headers,
        ..
    }: KafkaMessageAttributes = deluxe::extract_attributes(&mut ast)?;

    let header_impl = match HeaderType::from(headers) {
        HeaderType::CloudEvent => quote::quote! {
            fn headers(&self) -> ene_kafka::KafkaResult<ene_kafka::messages::kafka_message::Headers> {
                ene_kafka::messages::cloud_events::cloud_event::CloudEvent::cloud_event_headers(self)
            }
        },
        HeaderType::Empty => quote::quote! {
            fn headers(&self) -> ene_kafka::KafkaResult<ene_kafka::messages::kafka_message::Headers> {
                Ok(std::collections::HashMap::new())
            }
        },
    };
    // define impl variables
    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let key_ident = syn::Ident::new(&key.to_string(), struct_name.span());

    // generate
    Ok(quote::quote! {
        impl #impl_generics ene_kafka::messages::kafka_message::KafkaMessage<String, String> for #struct_name #type_generics #where_clause {
            fn topic(&self) -> ene_kafka::KafkaResult<ene_kafka::messages::kafka_message::KafkaTopic> {
                Ok(ene_kafka::messages::kafka_message::KafkaTopic {
                    name: #topic.to_string(),
                    content_type: ene_kafka::messages::kafka_message::ContentType::Json,
                })
            }

            fn payload(&self) -> ene_kafka::KafkaResult<String> {
                serde_json::to_string(self).map_err(|e| anyhow::anyhow!("Failed to serialize payload: {}", e))
            }

            fn key(&self) -> ene_kafka::KafkaResult<String> {
                Ok(self.#key_ident.to_string())
            }


            #header_impl
        }
    })
}
