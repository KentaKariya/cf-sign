use std::path::PathBuf;

use anyhow::Context;
use config::{FileFormat, FileSourceFile};
use serde::Deserialize;

use crate::Args;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "kentakariya";
const APPLICATION: &str = "cf-sign";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sign: Sign,
}

#[derive(Debug, Deserialize)]
pub struct Sign {
    pub duration: u64,
    pub key_id: String,
}

pub fn parse_options(overrides: &Args) -> anyhow::Result<Config> {
    let config_file: config::File<FileSourceFile, FileFormat> = get_config_file()?.into();

    config::Config::builder()
        .add_source(config_file)
        .set_override_option("sign.duration", overrides.duration)?
        .set_override_option("sign.key_id", overrides.key_id.clone())?
        .build()?
        .try_deserialize::<Config>()
        .context("Could not parse config file")
}

fn get_config_file() -> anyhow::Result<PathBuf> {
    directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .map(|d| d.config_dir().join("config.toml"))
        .context("Could not determine location for config file")
}
