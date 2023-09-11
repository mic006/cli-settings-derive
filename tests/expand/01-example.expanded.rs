item: "#[cli_settings_file = "#[serde_as]"]
#[cli_settings_clap =
"#[derive(clap::Parser)]#[command(version, about, long_about = None)]"] pub
struct Settings
{
    /// alpha setting explanation
    #[cli_settings_file] #[cli_settings_clap = "#[arg(long)]"] pub alpha :
    u32, /// beta setting explanation
    #[cli_settings_clap = "#[arg(short, long)]"] pub beta : String,
    /// gamma setting explanation
    #[cli_settings_file] pub gamma : u64,
}"
DBG main: pub struct Settings { pub alpha : u32, pub beta : String, pub gamma : u64 }
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate cli_settings_derive;
pub struct Settings {
    pub alpha: u32,
    pub beta: String,
    pub gamma: u64,
}
