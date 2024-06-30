use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(event_handler))]
struct HandlerAttributes {
    event: syn::ExprPath,
    handler: syn::Ident,
}

pub fn handler_derive_macro2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input.into())?;
    let HandlerAttributes { event, handler }: HandlerAttributes =
        deluxe::extract_attributes(&mut ast)?;
    let struct_name = &ast.ident;

    let event_path = event.path;

    Ok(quote::quote! {
        #[async_trait::async_trait]
        impl<InputEvent: ene_kafka::messages::cloud_events::cloud_event::CloudEvent<String, String>> EventHandler<InputEvent, #event_path> for #struct_name {
            fn event_type(&self) -> ene_kafka::KafkaResult<ene_kafka::messages::cloud_events::cloud_event::EventType> {
                use ene_kafka::messages::cloud_events::cloud_event::CloudEvent;
                #event_path::entity_event_type()
            }

            async fn handle(&self, event: &#event_path) -> ene_kafka::KafkaResult<()> {
                #struct_name::#handler(self, event).await
            }
        }
    })
}
