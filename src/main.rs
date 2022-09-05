use std::io::Read;

use anyhow::Ok;
use chrono::{Duration, Utc};
use clap::Parser;
use url::Url;

mod options;
mod sign;

#[cfg(test)]
mod tests;

/// Generate a signed CloudFront URL using the private key from stdin.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(value_parser = Url::parse)]
    url: Url,

    /// Validity in seconds
    #[clap(short, long)]
    duration: Option<u64>,

    /// CloudFront key ID
    #[clap(short, long)]
    key_id: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    let config: _ = options::parse_options(&args)?;

    let expire_at = Utc::now() + Duration::seconds(config.sign.duration as i64);

    let mut key = String::new();
    std::io::stdin().read_to_string(&mut key)?;

    sign::sign(&mut args.url, expire_at, &config.sign.key_id, &key)?;
    println!("{}", args.url);

    Ok(())
}
