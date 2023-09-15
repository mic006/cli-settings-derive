# cli-settings-derive

[![Crates.io](https://img.shields.io/crates/v/cli-settings-derive?logo=rust)](https://crates.io/crates/cli-settings-derive)
[![docs.rs](https://img.shields.io/docsrs/cli-settings-derive?logo=docs.rs)](https://docs.rs/cli-settings-derive)
[![CI](https://img.shields.io/github/actions/workflow/status/mic006/cli-settings-derive/ci.yml?branch=main)](https://github.com/mic006/cli-settings-derive)
[![Crates.io](https://img.shields.io/crates/l/cli-settings-derive)](https://crates.io/crates/cli-settings-derive)

Use a derive macro with annotations on your Command Line Interface settings struct
to manage the settings of your application:
- create an instance with default values (provided by annotations)
- read each possible configuration file, if it exists:
  - update the fields that are defined in the configuration file
- parse the command line arguments, and update the relevant fields with the provided argument

By using annotations, each field can be configurable via the configuration file(s) and/or the command line.

cli-settings-derive can be seen as a top layer above
- [serde](https://docs.rs/serde) for the file configuration parsing
- [clap](https://docs.rs/clap) for the command line parsing

See [doc.rs documentation](https://docs.rs/cli-settings-derive) for detailed documentation.

## Usage

- Define your own configuration structure.
- Add `#[cli_settings]` annotation to the struct
- Add `#[cli_settings_file = "xxx"]` annotation to provide the annotation for file parsing (serde)
- Add `#[cli_settings_clap = "xxx"]` annotation to provide the annotation for argument parsing (clap)
- For each field, also provide annotations (all are optional)
  - `#[cli_settings_default = "xxx"]` to set the default value.
  - `#[cli_settings_file = "xxx"]` to indicate that the field shall be read from the config
    file(s). The passed string if any will be an extra annotation to the file parsing struct.
  - `#[cli_settings_clap = "xxx"]` to indicate that the field shall be a command line argument.
    The passed string (if any) will be an extra annotation to the command line parsing struct.
- For each field, provide documentation (with ///) to generate the help message via clap.
- In your application code, call the Settings::build() method with the list of config files to read
  and the command line arguments to get your application configuration.

## Example

```rust
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(version, about, long_about = None)]"]
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

fn main() {
    /// Get the application configuration
    let cfg = Settings::build(
        vec![
            std::path::PathBuf::from("/path/to/system-config.yml"),
            std::path::PathBuf::from("/path/to/user-config.yml"),
        ],
        std::env::args_os(),
    ).unwrap();
}
```