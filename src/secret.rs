use anyhow::Result;
use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use hex::decode;
use std::env;

pub fn decrypt(input: &str) -> Result<String> {
    let password = env::var("UNAGI_PASSWORD")?;
    let password_bytes = password.as_bytes();

    let key = GenericArray::clone_from_slice(&password_bytes[0..16]);
    let cipher = Aes128Gcm::new(&key);

    let parts: Vec<&str> = input.split("::").collect();
    if parts.len() != 3 || parts[0] != "UNAGI" {
        return Err(anyhow::anyhow!("Invalid input format"));
    }

    let nonce_bytes = decode(parts[1])?;
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let ciphertext = decode(parts[2])?;

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => Ok(String::from_utf8(plaintext)?),
        Err(_) => Err(anyhow::anyhow!("Decryption failed")),
    }
}

pub fn api_password() -> Result<String> {
    Ok(decrypt("UNAGI::76254259ff4e8fd2a63b360b::3fd40d3a3cf282fedcbe55d790e9e7a48f506b63804f9a56")?)
}
