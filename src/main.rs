use std::{io::Read, path::PathBuf};

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand, Args};
use url::Url;

mod options;
mod sign;
mod upload;

#[cfg(test)]
mod tests;

/// Generate a signed CloudFront URL using the private key from stdin.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Validity in seconds
    #[clap(short, long)]
    duration: Option<u64>,

    /// CloudFront key ID
    #[clap(short, long)]
    key_id: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a signed URL for an existing resource
    Sign(SignCommand),

    /// Upload a file to S3 and generate a signed URL to it
    Upload(UploadCommand),
}

#[derive(Debug, Args)]
pub struct SignCommand {
    #[clap(value_parser = Url::parse)]
    url: Url,

}

#[derive(Debug, Args)]
pub struct UploadCommand {
    file: PathBuf,

    #[clap(short, long)]
    bucket: Option<String>,

    #[clap(short, long)]
    prefix: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    let config: _ = options::parse_options(&cli)?;

    let expire_at = Utc::now() + Duration::seconds(config.sign.duration as i64);
    let mut key = String::new();
    std::io::stdin().read_to_string(&mut key)?;

    match &mut cli.command {
        Command::Sign(s) => {
            sign::sign(&mut s.url, expire_at, &config.sign.key_id, &key)?;
            println!("{}", s.url);
        }
        Command::Upload(u) => {
            upload::upload(&u.file, &config.upload.bucket, &config.upload.prefix).await?;
        }
    }

    Ok(())
}

