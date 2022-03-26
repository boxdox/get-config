use anyhow::{bail, Result, Context};
use inquire::Text;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub gist_id: String,
    pub token: Option<String>,
}

pub fn get_config(package_name: &str) -> Result<Config> {
    let mut cfg: Config = confy::load(package_name).unwrap_or_default();

    if cfg.gist_id.is_empty() {
        let gist_id =
            Text::new("please enter a gist_id (this will be saved into a config file)").prompt()?;
        if gist_id.is_empty() {
            bail!("gist id is empty, cannot proceed. re-run this command and enter a valid gist id")
        }
        cfg.gist_id = gist_id;
        let token = Text::new("enter a token for github (optional)")
            .with_default("")
            .prompt()?;
        cfg.token = Some(token);
        confy::store(package_name, &cfg).with_context(|| "failed to save config")?;
    }

    Ok(cfg)
}

pub fn clear_config(package_name: &str) -> Result<(), confy::ConfyError> {
    let cfg = Config {
        gist_id: "".to_string(),
        token: None,
    };
    confy::store(package_name, cfg)
}
