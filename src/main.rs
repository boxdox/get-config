use std::env;
use crate::config::{get_config, clear_config};

mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let package_name = env!("CARGO_PKG_NAME");

    // check if `init` is passed, if yes, reset the config
    match env::args().nth(1) {
        Some(arg) => {
            if arg.eq("init") {
                clear_config(package_name)?;
            }
        },
        None => {}
    };

    let cfg = get_config(package_name)?;

    dbg!(cfg);

    Ok(())
}
