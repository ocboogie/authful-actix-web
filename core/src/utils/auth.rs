use argonautica::{input::SecretKey, Error, Hasher, Verifier};
use hex::encode;
use sha2::{Digest, Sha256};

// lazy_static::lazy_static! {
//     pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
// }

pub fn hash_session_id(id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(id);

    encode(hasher.result())
}

pub fn hash_password<'a, SK>(secret: SK, password: &'a str) -> Result<String, Error>
where
    SK: Into<SecretKey<'a>>,
{
    Hasher::<'a>::default()
        .with_password(password)
        .with_secret_key(secret)
        .hash()
}

pub fn verify_password<'a, SK>(secret: SK, hash: &'a str, password: &'a str) -> Result<bool, Error>
where
    SK: Into<SecretKey<'a>>,
{
    Verifier::<'a>::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret)
        .verify()
}
