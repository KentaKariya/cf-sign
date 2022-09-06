# 🔐 cf-sign
A simple CLI tool to generate signed URLs for Amazon CloudFront

## Features
* Create a signed URL to an existing CloudFront asset
* Upload a file to S3 and create a CloudFront signed URL to it

## ✅ Prerequisites
Before generating signed URLs for CloudFront, you need to register a key pair on your AWS account.
Refer to the [official documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-signed-urls.html) for details.

## 💾 Installation
Clone this repository and run `cargo install --path .` in the directory.
The compiled binary will be installed to `~/.cargo/bin/`.
Make sure the directory is on your `$PATH`.

## 🧰 Usage
The following configuration file is required to use this tool and must be placed at `~/.config/cf-sign/config.toml`: 

```toml
[sign]
# Duration in seconds for which the URL is valid
duration = 3600

# CloudFront key ID
key_id = "DUMMYKEY123"

[upload]
# CloudFront distribution base URL
url = "https://example.com"

# S3 bucket name
bucket = "my-s3-bucket"

# S3 bucket region
region = "us-east-1"

# S3 key prefix for the uploaded file
prefix = "upload/"
```

These options can be selectively overridden using CLI options.
Refer to the help message displayed by running `cf-sign --help` for further details.

Pass the private key for signing through stdin.

