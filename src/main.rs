use std::io::Read;

use anyhow::Ok;
use clap::Parser;
use url::Url;
use chrono::{Utc, Duration};

/// Generate a signed CloudFront URL using the private key from stdin.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(value_parser = Url::parse)]
    url: Url,

    /// Validity in seconds
    #[clap(short, long, default_value_t = 3600)]
    duration: u64,

    /// CloudFront key ID
    #[clap(short, long)]
    key_id: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let expire_at = Utc::now() + Duration::seconds(args.duration as i64);
    let mut key = String::new();
    std::io::stdin().read_to_string(&mut key)?;


    let signed_url = sign::sign(args.url.as_ref(), expire_at, &args.key_id, &key)?;
    println!("{}", signed_url);

    Ok(())
}

