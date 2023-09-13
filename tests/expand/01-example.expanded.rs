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
