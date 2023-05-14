use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Fields};

#[proc_macro_derive(cli_settings)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let cl = input.ident;
    let cl_builder = format_ident!("{}Builder", cl);

    quote! {
        struct #cl_builder {}
    }
    .into()
}
