#[macro_use]
extern crate cli_settings_derive;

//#[derive(PartialEq, Debug)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(version, about, long_about = None)]"]
pub struct Settings {
    /// alpha setting explanation
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(long)]"]
    pub alpha: u32,

    /// beta setting explanation
    #[cli_settings_default = "\"beta default value\".to_string()"]
    #[cli_settings_clap = "#[arg(short, long)]"]
    pub beta: String,

    /// gamma setting explanation
    #[cli_settings_default = "1 << 63"]
    #[cli_settings_file]
    pub gamma: u64,
}
