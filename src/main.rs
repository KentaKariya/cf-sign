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
    /// URL to sign
    #[clap(value_parser = Url::parse)]
    url: Url,

}

#[derive(Debug, Args)]
pub struct UploadCommand {
    /// Path to target
    file: PathBuf,

    /// Base URL of the CloudFront distribution
    #[clap(short, long)]
    url: Option<String>,

    /// S3 bucket name
    #[clap(short, long)]
    bucket: Option<String>,

    /// S3 bucket region
    #[clap(short, long)]
    region: Option<String>,

    /// S3 key prefix for uploaded file
    #[clap(short, long)]
    prefix: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let options = options::parse_options(&cli)?;

    let expire_at = Utc::now() + Duration::seconds(options.sign.duration as i64);
    let mut key = String::new();
    std::io::stdin().read_to_string(&mut key)?;

    let mut url = match &cli.command {
        Command::Sign(s) => s.url.clone(),
        Command::Upload(u) => upload::upload(&options.upload, &u.file).await?,
    };

    sign::sign(&mut url, expire_at, &options.sign.key_id, &key)?;
    println!("{}", url.as_str());

    Ok(())
}

