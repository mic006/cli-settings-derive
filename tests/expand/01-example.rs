#[macro_use]
extern crate cli_settings_derive;

//#[derive(PartialEq, Debug)]
#[cli_settings]
#[serde_as]
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Settings {
    /// alpha setting explanation
    #[arg(long)]
    #[file]
    pub alpha: u32,

    /// beta setting explanation
    #[arg(short, long)]
    pub beta: String,

    /// gamma setting explanation
    #[file]
    pub gamme: u64,
}
