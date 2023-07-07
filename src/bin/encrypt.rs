use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes128Gcm;
use hex::{encode, decode};
use rand::Rng;
use std::env;

fn main() {
    // UNAGI_PASSWORDから共通のパスワードを取得
    let password = env::var("UNAGI_PASSWORD").expect("UNAGI_PASSWORD is not set");
    
    // パスワードをbytesに変換
    let password_bytes = password.as_bytes();

    // 最初の16バイトをキーとして使用
    let key = GenericArray::clone_from_slice(&password_bytes[0..16]);
    let cipher = Aes128Gcm::new(&key);

    // ランダムな12バイトのnonceを生成
    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let nonce = GenericArray::from_slice(&nonce_bytes);

    // コマンドラインの第一引数からテキストを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments, expected a string to encrypt");
    }
    let plaintext = &args[1];
    let plaintext_bytes = plaintext.as_bytes();

    // テキストの暗号化
    let ciphertext = cipher.encrypt(nonce, plaintext_bytes).expect("Encryption failed");

    println!("Ciphertext: UNAGI::{}::{}", encode(&nonce_bytes), encode(&ciphertext));
}
