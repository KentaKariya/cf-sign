use chrono::{Duration, Utc};
use openssl::{pkey::PKey, sign::Signer, hash::MessageDigest, base64::encode_block};
use serde_json::json;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum SigningError {
    #[error("Malformed private key")]
    MalformedKey(#[from] openssl::error::ErrorStack),
    #[error("Malformed URL")]
    MalformedURL(#[from] url::ParseError),
}

type SigningResult<T> = Result<T, SigningError>;

fn b64_encode(data: &[u8]) -> String {
    encode_block(data)
        .chars()
        .map(|c| match c {
            '+' => '-',
            '=' => '_',
            '/' => '~',
            _ => c,
        })
        .collect()
}

fn derive_signature(payload: &str, key: &str) -> SigningResult<String> {
    let pk = PKey::private_key_from_pem(key.as_bytes())?;
    let mut signer = Signer::new(MessageDigest::sha1(), &pk)?;

    Ok(signer.sign_oneshot_to_vec(payload.as_bytes()).map(|x| b64_encode(&x))?)
}

fn create_policy(resource: &str, timestamp: i64) -> String {
    json!({
        "Statement": [
            {
                "Resource": resource,
                "Condition": {
                    "DateLessThan": {
                        "AWS:EpochTime": timestamp,
                    }
                }
            }
        ]
    }).to_string()
}

pub fn sign(resource: &mut Url, duration: u64, key_id: &str, key: &str) -> SigningResult<()> {
    let timestamp = (Utc::now() + Duration::seconds(duration as i64)).timestamp();
    let policy = create_policy(resource.as_ref(), timestamp);

    let signature = derive_signature(&policy, key)?;

    resource.query_pairs_mut()
        .append_pair("Expires", &timestamp.to_string())
        .append_pair("Signature", &signature)
        .append_pair("Key-Pair-Id", key_id)
        .finish();

    Ok(())
}

