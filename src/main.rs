use std::io::Read;

use anyhow::Ok;
use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(value_parser = Url::parse)]
    url: Url,

    #[clap(short, long, default_value_t = 60)]
    duration: u64,

    #[clap(short, long)]
    key_id: String,
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    let mut key = String::new();
    std::io::stdin().read_to_string(&mut key)?;
    sign::sign(&mut args.url, args.duration, &args.key_id, &key)?;

    println!("{}", args.url.as_ref());

    Ok(())
}

