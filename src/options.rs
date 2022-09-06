use std::path::PathBuf;

use anyhow::Context;
use config::{FileFormat, FileSourceFile, ConfigBuilder, builder::DefaultState};
use serde::Deserialize;

use crate::{Cli, UploadCommand, Command};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "kentakariya";
const APPLICATION: &str = "cf-sign";

type ConfigFile = config::File<FileSourceFile, FileFormat>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sign: Sign,
    pub upload: Upload,
}

#[derive(Debug, Deserialize)]
pub struct Sign {
    pub duration: u64,
    pub key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    pub bucket: String,
    pub prefix: String,
}

pub fn parse_options(overrides: &Cli) -> anyhow::Result<Config> {
    let config_file: ConfigFile = get_config_file()?.into();
    let mut config_builder = config::Config::builder()
        .add_source(config_file)
        .set_override_option("sign.duration", overrides.duration)?
        .set_override_option("sign.key_id", overrides.key_id.clone())?;

    if let Command::Upload(cmd) = &overrides.command {
        config_builder = override_upload_options(config_builder, cmd)?;
    }

    config_builder
        .build()?
        .try_deserialize::<Config>()
        .context("Could not parse config file")
}

fn override_upload_options(config_builder: ConfigBuilder<DefaultState>, cmd: &UploadCommand) -> anyhow::Result<ConfigBuilder<DefaultState>> {
    config_builder
        .set_override_option("upload.bucket", cmd.bucket.clone())?
        .set_override_option("upload.prefix", cmd.prefix.clone())
        .context("Could not override upload options")
}

fn get_config_file() -> anyhow::Result<PathBuf> {
    directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .map(|d| d.config_dir().join("config.toml"))
        .context("Could not determine location for config file")
}

