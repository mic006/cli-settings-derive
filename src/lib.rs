//! Manage CLI settings via config files and command line parsing.

use std::str::FromStr;

use let_or_return::let_or_return;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned};

/// Map of attributes by category
/// 'cli_settings_default': default field value
/// 'cli_settings_file': config file related attributes
/// 'cli_settings_clap': command line related attributes
/// 'doc': doc related attributes
/// '_': other attributes
type AttrMap = std::collections::HashMap<String, proc_macro2::TokenStream>;

/// Field element, quite similar to syn::Field, keeping only the relevant fields
struct Field<'a> {
    attrs: AttrMap,           // classified attributes of the field
    vis: &'a syn::Visibility, // field visibility
    ident: &'a syn::Ident,    // field name
    ty: &'a syn::Type,        // field type
                              //opt: bool,                // whether the type shall be converted to Option<ty>
}

/// Container for the whole settings struct
struct SettingStruct<'a> {
    s: &'a syn::ItemStruct, // associated syn::ItemStruct object
    attrs: AttrMap,         // classified attributes of the struct
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
            };
            // TODO update opt; based on what ?? which use case ? avoid optional of optional ???? => No: mandatory field for clap for subsommands
            ss.fields.push(f);
        }

        Ok(ss)
    }

    /// Classify a list of attributes, related to file , clap, or other
    fn classify_attributes(attrs: &'a Vec<syn::Attribute>) -> Result<AttrMap, syn::Error> {
        let mut res: AttrMap = Default::default();
        for attr in attrs {
            let (path, value) = match &attr.meta {
                syn::Meta::Path(p) => (Some(p), None),
                syn::Meta::NameValue(v) => (Some(&v.path), Some(&v.value)),
                _ => (None, None),
            };
            let mut handled_attr = false;
            if let Some(p) = path {
                if let Some(path_ident) = p.get_ident() {
                    let path_ident_str = path_ident.to_string();
                    if path_ident_str == "doc" {
                        handled_attr = true;
                        res.entry("doc".to_string())
                            .or_default()
                            .extend(attr.to_token_stream());
                    } else if path_ident_str.starts_with("cli_settings_") {
                        handled_attr = true;
                        if value.is_none() {
                            res.entry(path_ident_str).or_default();
                        } else if let Some(syn::Expr::Lit(syn::ExprLit {
                            attrs: _,
                            lit: syn::Lit::Str(l),
                        })) = value
                        {
                            res.entry(path_ident_str)
                                .or_default()
                                .extend(proc_macro2::TokenStream::from_str(&l.value())?);
                        } else {
                            return Err(syn::Error::new(attr.span(), "invalid attribute format"));
                        }
                    }
                }
            }
            if !handled_attr {
                // other attribute (including doc), keep as is
                res.entry("_".to_string())
                    .or_default()
                    .extend(attr.to_token_stream());
            }
        }
        Ok(res)
    }

    /// Output struct with given selection
    fn output_struct(
        &self,
        prefix: &str,
        field_filter: Option<&str>,
        attr_keys: &[&str],
    ) -> proc_macro2::TokenStream {
        let empty = proc_macro2::TokenStream::new();
        let (field_ty_start, field_ty_end) = if prefix.is_empty() {
            // no prefix, field with configured type
            (empty.clone(), empty.clone())
        } else {
            // prefix, field with Option<configured type>
            (
                proc_macro2::TokenStream::from_str("Option<").unwrap(),
                proc_macro2::TokenStream::from_str(">").unwrap(),
            )
        };
        // struct tokens
        let attrs = attr_keys
            .iter()
            .map(|k| self.attrs.get(*k).unwrap_or(&empty))
            .collect::<Vec<_>>();
        let vis = &self.s.vis;
        let struct_token = &self.s.struct_token;
        let name = format!("{}{}", prefix, self.s.ident);
        let ident = syn::Ident::new(&name, self.s.ident.span());
        // all fields tokens
        let fields = self
            .fields
            .iter()
            .filter(|f| {
                if let Some(k) = field_filter {
                    f.attrs.contains_key(k)
                } else {
                    true
                }
            })
            .map(|f| {
                // field tokens
                let field_attrs = attr_keys
                    .iter()
                    .map(|k| f.attrs.get(*k).unwrap_or(&empty))
                    .collect::<Vec<_>>();
                let field_vis = f.vis;
                let field_ident = f.ident;
                let field_ty = f.ty;
                // output one field (without separator)
                quote! {
                    #(#field_attrs)* #field_vis #field_ident: #field_ty_start #field_ty #field_ty_end
                }
            })
            .collect::<Vec<_>>();
        // output the whole struct
        quote! {
            #(#attrs)* #vis #struct_token #ident
            {
                #(#fields),*
            }
        }
    }

    /// Output the main structure
    fn output_main_struct(&self) -> proc_macro2::TokenStream {
        self.output_struct("", None, &["_", "doc"])
    }
    /// Output the file structure
    fn output_file_struct(&self) -> proc_macro2::TokenStream {
        self.output_struct("File", Some("cli_settings_file"), &["cli_settings_file"])
    }
    /// Output the clap structure
    fn output_clap_struct(&self) -> proc_macro2::TokenStream {
        self.output_struct(
            "Clap",
            Some("cli_settings_clap"),
            &["doc", "cli_settings_clap"],
        )
    }

    /// Output Default implementation for the main struct
    fn output_main_struct_default(&self) -> proc_macro2::TokenStream {
        let default = proc_macro2::TokenStream::from_str("Default::default()").unwrap();
        let ident = &self.s.ident;
        let fields = self
            .fields
            .iter()
            .map(|f| {
                let field_ident = f.ident;
                let field_default = f.attrs.get("cli_settings_default").unwrap_or(&default);
                // output one field (without separator)
                quote! {
                    #field_ident: #field_default
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl Default for #ident {
                fn default() -> Self {
                    Self{
                        #(#fields),*
                    }
                }
            }
        }
    }

    /// Output build() implementation for the main struct
    fn output_main_struct_build(&self) -> proc_macro2::TokenStream {
        let ident = &self.s.ident;
        quote! {
            impl #ident {
                pub fn build<F, I, T>(cfg_files: F, args: I) -> anyhow::Result<Self>
                where
                    F: IntoIterator<Item = std::path::PathBuf>,
                    I: IntoIterator<Item = T>,
                    T: Into<std::ffi::OsString> + Clone,
                {
                    let mut cfg = Self::default();
                    for file in cfg_files {
                        _cli_settings_derive::load_file(&file, &mut cfg)?;
                    }
                    _cli_settings_derive::parse_cli_args(args, &mut cfg)?;
                    Ok(cfg)
                }
            }
        }
    }

    /// Output update() implementation for the file struct
    fn output_struct_update(&self, prefix: &str, field_filter: &str) -> proc_macro2::TokenStream {
        let main_ident = &self.s.ident;
        let name = format!("{}{}", prefix, self.s.ident);
        let ident = syn::Ident::new(&name, self.s.ident.span());
        let fields = self
            .fields
            .iter()
            .filter(|f| f.attrs.contains_key(field_filter))
            .map(|f| {
                let field_ident = f.ident;
                // output one field (without separator)
                quote! {
                    if let Some(param) = self.#field_ident {
                        cfg.#field_ident = param;
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl #ident {
                fn update(self, cfg: &mut super::#main_ident) {
                    #(#fields)*
                }
            }
        }
    }
    /// Output the file struct update()
    fn output_file_struct_update(&self) -> proc_macro2::TokenStream {
        self.output_struct_update("File", "cli_settings_file")
    }
    /// Output the clap struct update()
    fn output_clap_struct_update(&self) -> proc_macro2::TokenStream {
        self.output_struct_update("Clap", "cli_settings_clap")
    }

    /// Output load_file() function
    fn output_load_file(&self) -> proc_macro2::TokenStream {
        let main_ident = &self.s.ident;
        let name = format!("File{}", self.s.ident);
        let ident = syn::Ident::new(&name, self.s.ident.span());
        quote! {
            pub fn load_file(path: &std::path::Path, cfg: &mut super::#main_ident) -> anyhow::Result<()> {
                // access file
                let file = std::fs::File::open(path);
                if let Err(err) = file {
                    if err.kind() == std::io::ErrorKind::NotFound {
                        // file not found is not a problem...
                        return Ok(());
                    }
                    // ... but everything else is -> propagate error
                    return Err(err).context(format!(
                        "Failed to open the configuration file '{}'",
                        path.display()
                    ));
                }
                let file = file.unwrap();

                // get parsed content
                let file_config: #ident = serde_yaml::from_reader(file).with_context(|| {
                    format!(
                        "Failed to parse the configuration file '{}'",
                        path.display()
                    )
                })?;

                // update config with content from the file
                file_config.update(cfg);

                Ok(())
            }
        }
    }

    /// Output parse_cli_args() function
    fn output_parse_cli_args(&self) -> proc_macro2::TokenStream {
        let main_ident = &self.s.ident;
        let name = format!("Clap{}", self.s.ident);
        let ident = syn::Ident::new(&name, self.s.ident.span());
        quote! {
            pub fn parse_cli_args<I, T>(args: I, cfg: &mut super::#main_ident) -> anyhow::Result<()>
            where
                I: IntoIterator<Item = T>,
                T: Into<std::ffi::OsString> + Clone,
            {
                let cli_args = #ident ::parse_from(args);
                cli_args.update(cfg);
                Ok(())
            }
        }
    }
}

/// # Description
///
/// Use a derive macro with annotations on your Command Line Interface settings struct
/// to manage the settings of your application:
/// - create an instance with default values (provided by annotations)
/// - read each possible configuration file, if it exists:
///   - update the fields that are defined in the configuration file
/// - parse the command line arguments, and update the relevant fields with the provided argument
///
/// By using annotations, each field can be configurable via the configuration file(s) and/or the command line.
///
/// cli-settings-derive can be seen as a top layer above
/// - [serde](https://docs.rs/serde) for the file configuration parsing
/// - [clap](https://docs.rs/clap) for the command line parsing
///
/// ## Usage
///
/// - Define your own configuration structure.
/// - Add `#[cli_settings]` annotation to the struct
/// - Add `#[cli_settings_file = "xxx"]` annotation to provide the annotation(s) for file parsing (serde)
/// - Add `#[cli_settings_clap = "xxx"]` annotation to provide the annotation(s) for argument parsing (clap)
/// - For each field, also provide annotations (all are optional)
///   - `#[cli_settings_default = "xxx"]` to set the default value.
///   - `#[cli_settings_file = "xxx"]` to indicate that the field shall be read from the config
///     file(s). The passed string if any will be extra annotation(s) to the file parsing struct.
///   - `#[cli_settings_clap = "xxx"]` to indicate that the field shall be a command line argument.
///     The passed string (if any) will be extra annotation(s) to the command line parsing struct.
/// - For each field, provide documentation (with ///) to generate the help message via clap.
/// - In your application code, call the Settings::build() method with the list of config files to read
///   and the command line arguments to get your application configuration.
///
/// ## Example
///
/// ```
/// use cli_settings_derive::cli_settings;
///
/// #[cli_settings]
/// #[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
/// #[cli_settings_clap = "#[derive(clap::Parser)]#[command(version, about)]"]
/// pub struct Settings {
///     /// alpha setting explanation
///     #[cli_settings_file]
///     #[cli_settings_clap = "#[arg(long)]"]
///     pub alpha: u32,
///
///     /// beta setting explanation, settable only from command line
///     #[cli_settings_default = "\"beta default value\".to_string()"]
///     #[cli_settings_clap = "#[arg(short, long)]"]
///     pub beta: String,
///
///     /// gamma setting explanation, settable only from config file
///     #[cli_settings_default = "1 << 63"]
///     #[cli_settings_file]
///     pub gamma: u64,
/// }
///
/// fn main() {
///     // Get the application configuration
///     let cfg = Settings::build(
///         vec![
///             std::path::PathBuf::from("/path/to/system-config.yml"),
///             std::path::PathBuf::from("/path/to/user-config.yml"),
///         ],
///         std::env::args_os(),
///     ).unwrap();
/// }
/// ```
///
/// A more complex example is available in the [crate repository](https://github.com/mic006/cli-settings-derive/blob/main/examples/example.rs), with:
/// - clap settings to tune the generated help message (-h)
/// - field with custom type and user provided function to parse the value from string
///
#[proc_macro_attribute]
pub fn cli_settings(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn_struct = parse_macro_input!(item as syn::ItemStruct);
    let ss = match SettingStruct::build(&syn_struct) {
        Ok(ss) => ss,
        Err(e) => return e.to_compile_error().into(),
    };

    let main_struct = ss.output_main_struct();
    let main_struct_default = ss.output_main_struct_default();
    let main_struct_build = ss.output_main_struct_build();
    let file_struct = ss.output_file_struct();
    let file_struct_update = ss.output_file_struct_update();
    let load_file = ss.output_load_file();
    let clap_struct = ss.output_clap_struct();
    let clap_struct_update = ss.output_clap_struct_update();
    let parse_cli_args = ss.output_parse_cli_args();

    quote! {
        #main_struct
        #main_struct_default
        #main_struct_build

        mod _cli_settings_derive {
            use anyhow::Context;
            use clap::Parser;
            use super::*;

            #file_struct
            #file_struct_update

            #load_file

            #clap_struct
            #clap_struct_update

            #parse_cli_args
        }
    }
    .into()
}
