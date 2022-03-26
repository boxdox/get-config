use inquire::{error::InquireError, Text};
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub gist_id: Option<String>,
    pub token: Option<String>,
}

pub fn get_config(package_name: &str) -> Result<Config, InquireError> {
    let mut cfg: Config = confy::load(package_name).unwrap_or_default();

    if cfg.gist_id.is_none() {
        let gist_id =
            Text::new("please enter a gist_id (this will be saved into a config file)").prompt()?;
        if gist_id.is_empty() {
            eprintln!(
                "gist id is empty, cannot proceed. re-run this command and enter a valid gist id"
            );
            process::exit(1);
        }
        cfg.gist_id = Some(gist_id);
        let token = Text::new("enter a token for github (optional)")
            .with_default("")
            .prompt()?;
        cfg.token = Some(token);
        confy::store(package_name, &cfg).unwrap();
    }

    Ok(cfg)
}

pub fn clear_config(package_name: &str) -> Result<(), confy::ConfyError> {
    let cfg = Config {
        gist_id: None,
        token: None,
    };
    confy::store(package_name, cfg)
}
