use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// Generates the Builder pattern implementation for a struct.
#[proc_macro_derive(DrawBuilder)]
pub fn derive_draw_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("DrawBuilder can only be derived for structs with named fields"),
        },
        _ => panic!("DrawBuilder can only be derived for structs"),
    };

    let setter_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        // This is the structure you requested (mut ref and setter):
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = #name;
                self
            }
        }
    });

    let expanded = quote! {
        impl<'a> DrawBuilder<'a, #struct_name> {
            #(#setter_methods)*
        }
    };

    expanded.into()
}
