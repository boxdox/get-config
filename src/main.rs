mod config;
mod github;
mod writer;

use crate::config::{clear_config, get_config};
use crate::github::fetch_gist;
use crate::writer::select_and_write_files;
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

    println!("fetching gist {}", gist_id);

    let files  = fetch_gist(&gist_id, Some(&token)).await?;

    select_and_write_files(&files, &token).await?;

    Ok(())
}
