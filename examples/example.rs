//! Example of cli_settings usage.

use cli_settings_derive::cli_settings;

#[derive(Debug)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(version, about = \"Example application for cli_settings\n\nLoad configuration files 'example1.yml' and 'example2.yml' from the current folder, process the command line arguments and finally display the resulting settings.\")]"]
pub struct Settings {
    /// alpha setting explanation
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(long)]"]
    pub alpha: u32,

    /// beta setting explanation, settable only from command line
    #[cli_settings_default = "\"beta default value\".to_string()"]
    #[cli_settings_clap = "#[arg(short, long)]"]
    pub beta: String,

    /// gamma setting explanation, settable only from config file
    #[cli_settings_default = "1 << 63"]
    #[cli_settings_file]
    pub gamma: u64,
}

fn main() -> anyhow::Result<()> {
    // Get the application configuration
    let cfg = Settings::build(
        vec![
            std::path::PathBuf::from("example1.yml"),
            std::path::PathBuf::from("example2.yml"),
        ],
        std::env::args_os(),
    )?;

    println!("{:#?}", cfg);
    Ok(())
}
