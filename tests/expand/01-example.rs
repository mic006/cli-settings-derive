#[macro_use]
extern crate cli_settings_derive;

#[derive(cli_settings)]
struct Settings {
    alpha: u32,
}
