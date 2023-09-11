use std::str::FromStr;

use let_or_return::let_or_return;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned};

/// Map of attributes by category
/// 'file': config file related attributes
/// 'clap': command line related attributes
/// '_': other attributes
type AttrMap<'a> = std::collections::HashMap<&'a str, proc_macro2::TokenStream>;

/// Field element, quite similar to syn::Field, keeping only the relevant fields
struct Field<'a> {
    attrs: AttrMap<'a>,       // classified attributes of the field
    vis: &'a syn::Visibility, // field visibility
    ident: &'a syn::Ident,    // field name
    ty: &'a syn::Type,        // field type
    opt: bool,                // whether the type shall be converted to Option<ty>
}

/// Container for the whole settings struct
struct SettingStruct<'a> {
    s: &'a syn::ItemStruct, // associated syn::ItemStruct object
    attrs: AttrMap<'a>,     // classified attributes of the struct
    fields: Vec<Field<'a>>, // list of fields
}

impl<'a> SettingStruct<'a> {
    /// Build SettingStruct from a syn::ItemStruct
    fn build(s: &'a syn::ItemStruct) -> Result<Self, syn::Error> {
        let mut ss = Self {
            s,
            attrs: Default::default(),
            fields: vec![],
        };

        let_or_return!(
            syn::Fields::Named(fields) = &s.fields,
            Err(syn::Error::new(
                s.span(),
                "only named structs are supported"
            ))
        );

        // struct attributes
        ss.attrs = Self::classify_attributes(&s.attrs)?;

        // fields
        ss.fields.reserve_exact(fields.named.len());
        for field in &fields.named {
            let f = Field {
                attrs: Self::classify_attributes(&field.attrs)?,
                vis: &field.vis,
                ident: field.ident.as_ref().ok_or_else(|| {
                    syn::Error::new(field.span(), "only named fields are supported")
                })?,
                ty: &field.ty,
                opt: false,
            };
            // TODO update opt; based on what ?? which use case ? avoid optional of optional ???? => No: mandatory field for clap for subsommands
            ss.fields.push(f);
        }

        Ok(ss)
    }

    /// Classify a list of attributes, related to file , clap, or other
    fn classify_attributes(attrs: &'a Vec<syn::Attribute>) -> Result<AttrMap<'a>, syn::Error> {
        let mut res: AttrMap = Default::default();
        for attr in attrs {
            let (path, value) = match &attr.meta {
                syn::Meta::Path(p) => (Some(p), None),
                syn::Meta::NameValue(v) => (Some(&v.path), Some(&v.value)),
                _ => (None, None),
            };
            let mut handled_attr = false;
            if let Some(p) = path {
                let v = if p.is_ident("cli_settings_file") {
                    Some("file")
                } else if p.is_ident("cli_settings_clap") {
                    Some("clap")
                } else {
                    None
                };
                if let Some(v) = v {
                    handled_attr = true;
                    if value.is_none() {
                        res.insert(v, proc_macro2::TokenStream::new());
                    } else if let Some(syn::Expr::Lit(syn::ExprLit {
                        attrs: _,
                        lit: syn::Lit::Str(l),
                    })) = value
                    {
                        res.insert(v, proc_macro2::TokenStream::from_str(l.suffix())?);
                    } else {
                        return Err(syn::Error::new(attr.span(), "invalid attribute format"));
                    }
                }
            }
            if !handled_attr {
                // other attribute (including doc), keep as is
                res.entry("_").or_default().extend(attr.to_token_stream());
            }
        }
        Ok(res)
    }

    /// Output the main structure   
    fn output_main(&self) -> proc_macro2::TokenStream {
        // struct tokens
        let empty = proc_macro2::TokenStream::new();
        let attr = self.attrs.get("_").unwrap_or(&empty);
        let vis = &self.s.vis;
        let struct_token = &self.s.struct_token;
        let ident = &self.s.ident;
        // all fields tokens
        let fields = self
            .fields
            .iter()
            .map(|f| {
                // field tokens
                let field_attr: &TokenStream = f.attrs.get("_").unwrap_or(&empty);
                let field_vis = f.vis;
                let field_ident = f.ident;
                let field_ty = f.ty;
                // output one field (without separator)
                quote! {
                    #field_attr #field_vis #field_ident: #field_ty
                }
            })
            .collect::<Vec<_>>();
        // output the whole struct
        quote! {
            #attr #vis #struct_token #ident
            {
                #(#fields),*
            }
        }
    }
}

#[proc_macro_attribute]
pub fn cli_settings(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    //println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    //let _attr = parse_macro_input!(attr as DeriveInput);

    let syn_struct = parse_macro_input!(item as syn::ItemStruct);
    let ss = SettingStruct::build(&syn_struct).expect("toto"); // TODO error report

    let main = ss.output_main();
    println!("DBG main: {}", main);
    main.into()

    // Debug code
    //let ts = proc_macro2::TokenStream::from_str("#[serde_as]#[trucmuche]").expect("toto");

    //syn::parse::Parser::parse_str(syn::Attribute::parse_outer, s);
    //    let toto = syn::parse_str::<syn::Attribute>("#[serde_as]#[derive(Toto)]").expect("toto");
    /*quote! {
        #ts
        #s
    }
    .into()*/
}
