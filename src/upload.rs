use std::path::PathBuf;

use anyhow::Context;
use aws_sdk_s3::types::ByteStream;

pub async fn upload(path: &PathBuf, bucket: &str, prefix: &str) -> anyhow::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let filename = path.file_name().and_then(|f| f.to_str()).context(format!("Illegal file name in path {:?}", path))?;
    let bs = ByteStream::from_path(path).await?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    client.put_object()
        .bucket(bucket)
        .key(prefix.to_owned() + filename)
        .content_type(mime.essence_str())
        .body(bs)
        .send()
        .await
        .context("Failed to upload file to S3")?;

    Ok(())
}

