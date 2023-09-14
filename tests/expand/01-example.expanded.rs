#[macro_use]
extern crate cli_settings_derive;
pub struct Settings {
    /// alpha setting explanation
    pub alpha: u32,
    /// beta setting explanation
    pub beta: String,
    /// gamma setting explanation
    pub gamma: u64,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            alpha: Default::default(),
            beta: "beta default value",
            gamma: 1 << 63,
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
            cli_settings_derive::load_file(&file, &mut cfg)?;
        }
        cli_settings_derive::parse_args(&args, &mut cfg)?;
        Ok(cfg)
    }
}
mod cli_settings_derive {
    #[serde_as]
    pub struct FileSettings {
        pub alpha: Option<u32>,
        pub gamma: Option<u64>,
    }
    impl FileSettings {
        fn update(self, cfg: &mut super::Settings) -> Self {
            if let Some(param) = self.alpha {
                cfg.alpha = param;
            }
            if let Some(param) = self.gamma {
                cfg.gamma = param;
            }
        }
    }
    fn load_file(
        path: &std::path::Path,
        cfg: &mut super::Settings,
    ) -> anyhow::Result<()> {
        let file = std::fs::File::open(path);
        if let Err(err) = file {
            if err.kind() == std::io::ErrorKind::NotFound {
                return Ok(());
            }
            return Err(err)
                .context({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Failed to open the configuration file \'{0}\'", path
                            .display(),
                        ),
                    );
                    res
                });
        }
        let file = file.unwrap();
        let file_config: FileSettings = serde_yaml::from_reader(file)
            .with_context(|| {
                {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Failed to parse the configuration file \'{0}\'", path
                            .display(),
                        ),
                    );
                    res
                }
            })?;
        file_config.update(cfg);
        Ok(())
    }
    #[command(version, about, long_about = None)]
    pub struct ClapSettings {
        /// alpha setting explanation
        #[arg(long)]
        pub alpha: Option<u32>,
        /// beta setting explanation
        #[arg(short, long)]
        pub beta: Option<String>,
    }
    impl ClapSettings {
        fn update(self, cfg: &mut super::Settings) -> Self {
            if let Some(param) = self.alpha {
                cfg.alpha = param;
            }
            if let Some(param) = self.beta {
                cfg.beta = param;
            }
        }
    }
}
