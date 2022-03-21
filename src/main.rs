mod config;
mod github;

use crate::config::{clear_config, get_config};
use crate::github::fetch_gist;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let package_name = env!("CARGO_PKG_NAME");

    // check if `init` is passed, if yes, reset the config
    match env::args().nth(1) {
        Some(arg) => {
            if arg.eq("init") {
                clear_config(package_name)?;
            }
        }
        None => {}
    };

    let cfg = get_config(package_name)?;
    let gist_id = cfg.gist_id.unwrap();
    let token = cfg.token.unwrap();

    let response = fetch_gist(&gist_id, Some(&token)).await?;

    dbg!(response.keys());

    Ok(())
}
