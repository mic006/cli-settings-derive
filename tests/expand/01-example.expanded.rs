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
mod cli_settings_derive {
    #[command(version, about, long_about = None)]
    pub struct ClapSettings {
        /// alpha setting explanation
        #[arg(long)]
        pub alpha: Option<u32>,
        /// beta setting explanation
        #[arg(short, long)]
        pub beta: Option<String>,
    }
    #[serde_as]
    pub struct FileSettings {
        pub alpha: Option<u32>,
        pub gamma: Option<u64>,
    }
}
