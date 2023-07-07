use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use hex::decode;
use std::env;

fn main() {
    // UNAGI_PASSWORDから共通のパスワードを取得
    let password = env::var("UNAGI_PASSWORD").expect("UNAGI_PASSWORD is not set");
    
    // パスワードをbytesに変換
    let password_bytes = password.as_bytes();

    // 最初の16バイトをキーとして使用
    let key = GenericArray::clone_from_slice(&password_bytes[0..16]);
    let cipher = Aes128Gcm::new(&key);

    // コマンドラインの第一引数からテキストを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments, expected a string to decrypt");
    }
    let encrypted = &args[1];
    let parts: Vec<&str> = encrypted.split("::").collect();
    if parts.len() != 3 || parts[0] != "UNAGI" {
        panic!("Invalid input format");
    }

    // NonceとCiphertextを取得
    let nonce_bytes = decode(parts[1]).unwrap();
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let ciphertext = decode(parts[2]).unwrap();

    // テキストの復号化
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).expect("Decryption failed");
    println!("{}", String::from_utf8(plaintext).unwrap());
}
