//! Test usage of cli_settings with a simple but exhaustive example

#[macro_use]
extern crate cli_settings_derive;

#[derive(PartialEq, Debug)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(version)]"]
pub struct Settings {
    /// alpha setting explanation
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(long)]"]
    pub alpha: bool,

    /// beta setting explanation
    #[cli_settings_default = "\"beta default value\".to_string()"]
    #[cli_settings_clap = "#[arg(long)]"]
    pub beta: String,

    /// gamma setting explanation
    #[cli_settings_default = "1 << 63"]
    #[cli_settings_file]
    pub gamma: u64,

    /// delta setting explanation
    #[cli_settings_default = "42"]
    #[cli_settings_file]
    pub delta: u32,

    /// epsilon setting explanation
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long)]"]
    pub epsilon: u32,
}

/// Test default() method
#[test]
pub fn default() {
    let expected = Settings {
        alpha: false,
        beta: "beta default value".to_string(),
        gamma: 1 << 63,
        delta: 42,
        epsilon: 0,
    };
    assert_eq!(Settings::default(), expected);
}

/// Test build() method
/// Load several files, then command line arguments, and check that the overall config is the expected one
#[test]
pub fn build() -> anyhow::Result<()> {
    let expected = Settings {
        alpha: false,                  // default value
        beta: "something".to_string(), // set by command line
        gamma: 728,                    // set by one config file
        delta: 32,                     // set by several config files
        epsilon: 1024,                 // set by config file and command line
    };

    let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let test_dir = std::path::Path::new(&project_dir).join("tests");

    let cfg = Settings::build(
        vec![
            test_dir.join("usage1.yml"),
            test_dir.join("usage-does-not-exist.yml"),
            test_dir.join("usage2.yml"),
        ],
        &["test-bin", "--beta=something", "-e", "1024"],
    )?;
    assert_eq!(cfg, expected);
    Ok(())
}
