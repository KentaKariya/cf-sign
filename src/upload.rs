use std::path::PathBuf;

use anyhow::Context;
use aws_sdk_s3::{types::ByteStream, Region};
use url::Url;

use crate::options;

pub async fn upload(config: &options::Upload, path: &PathBuf) -> anyhow::Result<Url> {
    let client = create_client(config).await;
    let s3_key = derive_s3_key(&config.prefix, path)?;
    let bs = ByteStream::from_path(path).await?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    client.put_object()
        .bucket(&config.bucket)
        .key(&s3_key)
        .content_type(mime.essence_str())
        .body(bs)
        .send()
        .await
        .context("Failed to upload file to S3")?;

    Url::parse(&config.url)
        .context(format!("Invalid URL {}", &config.url))?
        .join(&s3_key)
        .context(format!("Could not create URL for {}", &s3_key))
}

pub fn derive_s3_key(prefix: &str, path: &PathBuf) -> anyhow::Result<String> {
    let filename = path.file_name().and_then(|f| f.to_str()).context(format!("Illegal file name in path {:?}", path))?;
    Ok(prefix.to_owned() + filename)
}

async fn create_client(config: &options::Upload) -> aws_sdk_s3::Client {
    let sdk_config = match &config.region {
        Some(r) => aws_config::from_env().region(Region::new(r.clone())).load().await,
        None => aws_config::load_from_env().await,
    };

    aws_sdk_s3::Client::new(&sdk_config)
}

