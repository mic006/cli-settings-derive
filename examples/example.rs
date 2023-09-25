//! Example of cli_settings usage.

use cli_settings_derive::cli_settings;

/// Type for custom field, with custom parsing
#[derive(Default, PartialEq, Debug, Clone, serde_with::DeserializeFromStr)]
pub struct MemSize {
    pub nb: u64,
}

/// Convert memory size from string with optional SI prefix to a value (kibi mode)
impl std::str::FromStr for MemSize {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ch = s.chars().peekable();

        let mut si_unit = 0;
        let mut value = 0u64;

        // consume all digits
        while let Some(c) = ch.peek() {
            match *c {
                '0'..='9' => value = 10 * value + (*c as u64 - '0' as u64),
                ' ' => (),  // ignore spaces
                _ => break, // stop on any other char
            }
            let _ = ch.next();
        }

        // consume any SI prefix
        if let Some(c) = ch.peek() {
            match *c {
                'k' | 'K' => si_unit = 1,
                'm' | 'M' => si_unit = 2,
                'g' | 'G' => si_unit = 3,
                't' | 'T' => si_unit = 4,
                'p' | 'P' => si_unit = 5,
                'e' | 'E' => si_unit = 6,
                _ => (),
            }
            if si_unit != 0 {
                let _ = ch.next();
            }
        }

        // compute final value
        if si_unit != 0 {
            value <<= 10 * si_unit;
        }

        Ok(Self { nb: value })
    }
}

/// User defined enum
#[derive(clap::ValueEnum, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
}

/// External enum parsing
fn parse_log_level(input: &str) -> Result<log::LevelFilter, &'static str> {
    match input.to_lowercase().as_str() {
        "off" => Ok(log::LevelFilter::Off),
        "error" => Ok(log::LevelFilter::Error),
        "warn" => Ok(log::LevelFilter::Warn),
        "info" => Ok(log::LevelFilter::Info),
        "debug" => Ok(log::LevelFilter::Debug),
        "trace" => Ok(log::LevelFilter::Trace),
        _ => {
            Err("expecting log level value [possible values: off, error, warn, info, debug, trace]")
        }
    }
}

/// Example application for cli_settings
///
/// Load configuration files 'example1.yml' and 'example2.yml' from the current folder,
/// process the command line arguments and finally display the resulting settings.
#[derive(Debug)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(name = \"cli-settings-example\", version)]"]
pub struct Settings {
    /// Buffer size in bytes, can be expressed with SI suffix like 64k for 64*1024 bytes
    #[cli_settings_default = "\"4k\".parse().expect(\"constant\")"]
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, value_name=\"SIZE\", global = true)]"]
    pub buffer_size: MemSize,

    /// alpha setting explanation
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long)]"]
    pub alpha: u32,

    /// beta setting explanation, settable only from command line
    #[cli_settings_default = "\"beta default value\".to_string()"]
    #[cli_settings_clap = "#[arg(long, global = true)]"]
    pub beta: String,

    /// gamma setting explanation, settable only from config file
    #[cli_settings_default = "1 << 63"]
    #[cli_settings_file]
    pub gamma: u64,

    /// choose home planet
    #[cli_settings_default = "Planet::Earth"]
    #[cli_settings_clap = "#[arg(short, long, global = true)]"]
    #[cli_settings_file]
    pub planet: Planet,

    /// minimum logging level to output
    #[cli_settings_default = "log::LevelFilter::Info"]
    #[cli_settings_clap = "#[arg(short, long, value_parser = parse_log_level, value_name=\"LEVEL\", global = true)]"]
    #[cli_settings_file = "#[serde_as(as = \"Option<serde_with::DisplayFromStr>\")]"]
    pub log: log::LevelFilter,

    #[cli_settings_mandatory]
    #[cli_settings_default = "CliCommand::Show"]
    #[cli_settings_clap = "#[command(subcommand)]"]
    pub command: CliCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum CliCommand {
    /// Show current configuration
    ///
    /// Show the configuration that will be applied
    Show,
    /// Load some file (fake)
    ///
    /// Some detailed explanation, displayed via `example help load`
    Load { file: std::path::PathBuf },
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
