use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use anyhow::Result;
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

///////////////////////////////////////////////////////////////////////////////
// Embedded secrets
//
// Create secrets by:
// $ cargo run --bin encrypt "secret"
///////////////////////////////////////////////////////////////////////////////

pub fn api_password() -> Result<String> {
    decrypt("UNAGI::76254259ff4e8fd2a63b360b::3fd40d3a3cf282fedcbe55d790e9e7a48f506b63804f9a56")
}

pub fn api_token() -> Result<String> {
    decrypt(
        "UNAGI::7cc16a775814eb937770b6fc::\
        70970bb8e6a0012d6639fe9db87e87b42a793f925d8ce5ef940ef9ae8aafe0fc\
        7001aca64826796bfe3a297afaa667182b625e9e00daa31e5c763ce160c84739\
        b8e2738292bc988a64d542eca09b1020cbc3f44b0b8c1682d536e9edb073e667\
        21c8f00148f0bea3384a26b889db6e3758f18199b046859da4621234e2504fd5\
        d3b104152232b5ac22343a6ac87824ad4dbbfbdabeee3218408dcfe3c416a595\
        39b16248c39ee18d40477732aad5f930b88f1e8cc3b5cc5957980f5c",
    )
}
