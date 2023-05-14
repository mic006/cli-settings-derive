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

See [doc.rs documentation](https://docs.rs/cli-settings-derive) for detailed documentation and an example.