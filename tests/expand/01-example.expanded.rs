#[macro_use]
extern crate cli_settings_derive;
/// Application summary (visible with -h)
///
/// Application long description (visible with --help)
pub struct Settings {
    /// alpha setting explanation
    pub alpha: u32,
    /// beta setting explanation
    pub beta: String,
    /// gamma setting explanation
    pub gamma: u64,
    /// mandatory argument
    pub path: std::path::PathBuf,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            alpha: Default::default(),
            beta: "beta default value".to_string(),
            gamma: 1 << 63,
            path: Default::default(),
        }
    }
}
impl Settings {
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
mod _cli_settings_derive {
    use anyhow::Context;
    use clap::Parser;
    use super::*;
    struct FileSettings {
        pub alpha: Option<u32>,
        pub gamma: Option<u64>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for FileSettings {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "alpha" => _serde::__private::Ok(__Field::__field0),
                            "gamma" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"alpha" => _serde::__private::Ok(__Field::__field0),
                            b"gamma" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<FileSettings>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = FileSettings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct FileSettings",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<u32>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct FileSettings with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<u64>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct FileSettings with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(FileSettings {
                            alpha: __field0,
                            gamma: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<u32>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<u64>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("alpha"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<u32>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("gamma"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<u64>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("alpha")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("gamma")?
                            }
                        };
                        _serde::__private::Ok(FileSettings {
                            alpha: __field0,
                            gamma: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["alpha", "gamma"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "FileSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<FileSettings>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl FileSettings {
        fn update(self, cfg: &mut super::Settings) {
            if let Some(param) = self.alpha {
                cfg.alpha = param;
            }
            if let Some(param) = self.gamma {
                cfg.gamma = param;
            }
        }
    }
    pub fn load_file(
        path: &std::path::Path,
        cfg: &mut super::Settings,
    ) -> anyhow::Result<()> {
        let file = std::fs::File::open(path);
        if let Err(err) = file {
            if err.kind() == std::io::ErrorKind::NotFound {
                return Ok(());
            }
            return Err(err)
                .context(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to open the configuration file \'{0}\'", path
                                .display(),
                            ),
                        );
                        res
                    }),
                );
        }
        let file = file.unwrap();
        let file_config: FileSettings = serde_yaml::from_reader(file)
            .with_context(|| {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Failed to parse the configuration file \'{0}\'", path
                            .display(),
                        ),
                    );
                    res
                })
            })?;
        file_config.update(cfg);
        Ok(())
    }
    /// Application summary (visible with -h)
    ///
    /// Application long description (visible with --help)
    #[command(version)]
    struct ClapSettings {
        /// alpha setting explanation
        #[arg(long)]
        pub alpha: Option<u32>,
        /// beta setting explanation
        #[arg(short, long)]
        pub beta: Option<String>,
        /// mandatory argument
        pub path: std::path::PathBuf,
    }
    #[automatically_derived]
    #[allow(unused_qualifications, clippy::redundant_locals)]
    impl clap::Parser for ClapSettings {}
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::CommandFactory for ClapSettings {
        fn command<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("cli-settings-derive-tests");
            <Self as clap::Args>::augment_args(__clap_app)
        }
        fn command_for_update<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("cli-settings-derive-tests");
            <Self as clap::Args>::augment_args_for_update(__clap_app)
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for ClapSettings {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = ClapSettings {
                alpha: __clap_arg_matches.remove_one::<u32>("alpha"),
                beta: __clap_arg_matches.remove_one::<String>("beta"),
                path: __clap_arg_matches
                    .remove_one::<std::path::PathBuf>("path")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: path",
                    ))?,
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if __clap_arg_matches.contains_id("alpha") {
                #[allow(non_snake_case)]
                let alpha = &mut self.alpha;
                *alpha = __clap_arg_matches.remove_one::<u32>("alpha");
            }
            if __clap_arg_matches.contains_id("beta") {
                #[allow(non_snake_case)]
                let beta = &mut self.beta;
                *beta = __clap_arg_matches.remove_one::<String>("beta");
            }
            if __clap_arg_matches.contains_id("path") {
                #[allow(non_snake_case)]
                let path = &mut self.path;
                *path = __clap_arg_matches
                    .remove_one::<std::path::PathBuf>("path")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: path",
                    ))?;
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Args for ClapSettings {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("ClapSettings"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("ClapSettings")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 3usize] = [
                                    clap::Id::from("alpha"),
                                    clap::Id::from("beta"),
                                    clap::Id::from("path"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("alpha")
                            .value_name("ALPHA")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("alpha setting explanation")
                            .long_help(None)
                            .long("alpha");
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("beta")
                            .value_name("BETA")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("beta setting explanation")
                            .long_help(None)
                            .short('b')
                            .long("beta");
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("path")
                            .value_name("PATH")
                            .required(true && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    std::path::PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("mandatory argument").long_help(None);
                        let arg = arg;
                        arg
                    });
                __clap_app
                    .about("Application summary (visible with -h)")
                    .long_about(
                        "Application summary (visible with -h)\n\nApplication long description (visible with --help)",
                    )
                    .version("0.0.0")
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("ClapSettings")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 3usize] = [
                                    clap::Id::from("alpha"),
                                    clap::Id::from("beta"),
                                    clap::Id::from("path"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("alpha")
                            .value_name("ALPHA")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("alpha setting explanation")
                            .long_help(None)
                            .long("alpha");
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("beta")
                            .value_name("BETA")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("beta setting explanation")
                            .long_help(None)
                            .short('b')
                            .long("beta");
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("path")
                            .value_name("PATH")
                            .required(true && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    std::path::PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("mandatory argument").long_help(None);
                        let arg = arg.required(false);
                        arg
                    });
                __clap_app
                    .about("Application summary (visible with -h)")
                    .long_about(
                        "Application summary (visible with -h)\n\nApplication long description (visible with --help)",
                    )
                    .version("0.0.0")
            }
        }
    }
    impl ClapSettings {
        fn update(self, cfg: &mut super::Settings) {
            if let Some(param) = self.alpha {
                cfg.alpha = param;
            }
            if let Some(param) = self.beta {
                cfg.beta = param;
            }
            cfg.path = self.path;
        }
    }
    pub fn parse_cli_args<I, T>(args: I, cfg: &mut super::Settings) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let cli_args = ClapSettings::parse_from(args);
        cli_args.update(cfg);
        Ok(())
    }
}
