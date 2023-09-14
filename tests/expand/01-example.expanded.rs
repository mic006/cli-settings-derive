DBG: impl Settings
{
    pub fn build < F, I, T > (cfg_files : F, args : I) -> anyhow :: Result <
    Self > where F : IntoIterator < Item = std :: path :: PathBuf >, I :
    IntoIterator < Item = T >, T : Into < std :: ffi :: OsString > + Clone,
    {
        let mut cfg = Self :: default() ; for file in cfg_files
        { cli_settings_derive :: load_file(& file, & mut cfg) ? ; }
        cli_settings_derive :: parse_args(& args, & mut cfg) ? ; Ok(cfg)
    }
}
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
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
    #[command(version, about, long_about = None)]
    pub struct ClapSettings {
        /// alpha setting explanation
        #[arg(long)]
        pub alpha: Option<u32>,
        /// beta setting explanation
        #[arg(short, long)]
        pub beta: Option<String>,
    }
}
