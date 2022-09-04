# 🔐 cf-sign
A simple CLI tool to generate signed URLs for Amazon CloudFront

## ✅ Prerequisites
Before generating signed URLs for CloudFront, you need to register a key pair on your AWS account.
Refer to the [official documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-signed-urls.html) for details.

## 💾 Installation
Clone this repository and run `cargo install --path .` in the directory.
The compiled binary will be installed to `~/.cargo/bin/`.
Make sure the directory is on your `$PATH`.

## 🧰 Usage
For the usage, refer to the help message shown by running `cf-sign --help`.

