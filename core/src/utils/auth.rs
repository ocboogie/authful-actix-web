use argonautica::{input::SecretKey, Error, Hasher, Verifier};
use hex::encode;
use sha2::{Digest, Sha256};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
    pub static ref ITERATIONS: u32 = std::env::var("HASHING_ITERATIONS")
        .ok()
        .and_then(|memory_size| memory_size.parse::<u32>().ok())
        .unwrap_or(192);
    pub static ref MEMORY_SIZE: u32 = std::env::var("HASHING_MEMORY_SIZE")
        .ok()
        .and_then(|memory_size| memory_size.parse::<u32>().ok())
        .unwrap_or(4096);
}

pub fn hash_session_id(id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(id);

    encode(hasher.result())
}

pub fn hash_password<'a>(password: &'a str) -> Result<String, Error> {
    Hasher::<'a>::default()
        .configure_iterations(*ITERATIONS)
        .configure_memory_size(*MEMORY_SIZE)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password<'a>(hash: &'a str, password: &'a str) -> Result<bool, Error> {
    Verifier::<'a>::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
}
