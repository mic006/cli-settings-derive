use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields, Item,
};

struct SettingStruct {
    s: syn::ItemStruct,
}

impl SettingStruct {
    fn build(s: syn::ItemStruct) -> Self {
        Self { s }
    }

    fn clone_with_suffix(&self, suffix: &str) -> Self {
        let mut c = self.clone();
        c.s.ident = format_ident!("{}{}", c.s.ident, suffix);
        c
    }

    /// Filter main attributes
    fn filter_main_attr(&mut self, keep_attrs: &[&str], keep_docs: bool) {
        self.s.attrs.retain(|e| {
            let p = e.path();
            if p.is_ident("doc") {
                keep_docs
            } else {
                for attr in keep_attrs {
                    if p.is_ident(attr) {
                        return true;
                    }
                }
                false
            }
        });
    }

    fn filter_fields_with_attr(&mut self, keep_attrs: &[&str], keep_docs: bool) {}
}
impl Clone for SettingStruct {
    fn clone(&self) -> Self {
        Self { s: self.s.clone() }
    }
}
impl quote::ToTokens for SettingStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.s.to_tokens(tokens);
    }
}
// impl From<SettingStruct> for proc_macro2::TokenStream {
//     fn from(value: SettingStruct) -> Self {
//         value.s.to_token_stream().into()
//     }
// }

#[proc_macro_attribute]
pub fn cli_settings(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    //println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    //let _attr = parse_macro_input!(attr as DeriveInput);
    let mut s = SettingStruct::build(parse_macro_input!(item as syn::ItemStruct));
    //println!("_attr: \"{:?}\"", _attr);
    //println!("_item: \"{:?}\"", _item);

    s.filter_main_attr(&["serde_as"], true);

    // let cl = &item.ident;
    // let cl_builder = format_ident!("{}Builder", cl);

    // quote! {
    //     #item
    //     struct #cl_builder {}
    // }
    // .into()
    quote! {
        #s
    }
    .into()
}
