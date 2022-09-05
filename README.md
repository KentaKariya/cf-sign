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
The following configuration file is required to use this tool and must be placed at `~/.config/cf-sign/config.toml`: 

```toml
[sign]
duration = 3600
key_id = "DUMMYKEY123"
```

These options can be selectively overridden using CLI options.
Refer to the help message displayed by running `cf-sign --help` for further details.

Pass the private key for signing through stdin.

