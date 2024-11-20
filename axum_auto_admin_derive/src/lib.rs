use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Ident};


fn impl_data_model_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_with_option = Ident::new(&format!("Edit{}", name), name.span());
    let name_with_id = Ident::new(&format!("WithID{}", name), name.span());


    let fields = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("DataModel can only be derived for structs with named fields."),
        },
        _ => panic!("DataModel can only be derived for structs."),
    };

    let original_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub #field_name: #field_type,
        }
    });

    let optional_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub #field_name: Option<#field_type>,
        }
    });

    let gen = quote! {
        impl DataModel for #name {
            fn get_struct_name() -> String {
                stringify!(#name).to_lowercase().to_string()
            }

            fn get_router() -> axum::Router {

                axum::Router::new()
                    .route(format!("/{}/list", Self::get_struct_name()).as_str(), axum::routing::get(Self::list_handler))
                    .route(format!("/{}/create", Self::get_struct_name()).as_str(), axum::routing::get(Self::create_handler))
                    .route(format!("/{}/:id/update", Self::get_struct_name()).as_str(), axum::routing::get(Self::update_handler))
                    .route(format!("/{}/:id/delete", Self::get_struct_name()).as_str(), axum::routing::get(Self::delete_handler))
            }

            async fn list_handler() -> impl axum::response::IntoResponse {
                tracing::info!("THis is the list page");
                "List Page"
            }

            async fn create_handler() -> impl axum::response::IntoResponse {
                tracing::info!("THis is the create page");
                "Create Page"
            }

            async fn update_handler() -> impl axum::response::IntoResponse {
                tracing::info!("THis is the update page");
                "Update Page"
            }

            async fn delete_handler() -> impl axum::response::IntoResponse {
                tracing::info!("THis is the delete page");
                "Delete Page"
            }
        }

        pub struct #name_with_option {
            #(#optional_fields)*
        }

        pub struct #name_with_id {
            pub id: i32,
            #(#original_fields)*
        }
    };

    gen.into()
}

#[proc_macro_derive(DataModel)]
pub fn data_model_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_data_model_macro(&ast)
}
