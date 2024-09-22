use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(cloud_event))]
struct CloudEventAttributes {
    content_type: String,
    #[deluxe(default = "1.0".to_string())]
    version: String,
    event_type: String,
    event_source: String,
    id: syn::Ident,
}

pub fn cloudevent_derive_macro2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input.into())?;
    let CloudEventAttributes {
        content_type,
        version,
        event_type,
        event_source,
        id,
    }: CloudEventAttributes = deluxe::extract_attributes(&mut ast)?;
    let struct_name = &ast.ident; 
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let id_ident = syn::Ident::new(&id.to_string(), struct_name.span());

    Ok(quote::quote! {
        impl #impl_generics ene_kafka::messages::cloud_events::cloud_event::CloudEvent<String, String> for #struct_name #type_generics #where_clause {
            fn spec_version(&self) -> ene_kafka::KafkaResult<String> {
                Ok(#version.to_string())
            }

            fn event_type(&self) -> ene_kafka::KafkaResult<String> {
                Ok(#event_type.to_string())
            }

            fn event_source(&self) -> ene_kafka::KafkaResult<String> {
                Ok(#event_source.to_string())
            }

            fn event_id(&self) -> ene_kafka::KafkaResult<String> {
                Ok(self.#id_ident.to_string())
            }

            fn event_time(&self) -> ene_kafka::KafkaResult<String> {
                Ok(chrono::Utc::now().to_rfc3339())
            }

            fn event_content_type(&self) -> ene_kafka::KafkaResult<String> {
                Ok(#content_type.to_string())
            }

            fn entity_event_type() -> ene_kafka::KafkaResult<String> {
                Ok(#event_type.to_string())
            }
        }
    })
}
