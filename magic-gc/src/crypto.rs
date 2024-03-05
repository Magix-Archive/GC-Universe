use base64::{Engine, prelude::BASE64_STANDARD};
use bcrypt::{DEFAULT_COST, hash, verify};
use rand::random;
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt};
use sha256::digest;

static RSA_KEY: &str = include_str!("../resources/private_key.pem");

/// RSA decrypts the password using RSA/ECB/PKCS1Padding.
/// password: The encrypted password in Base64.
pub fn decrypt_password(password: String) -> String {
    let password = BASE64_STANDARD.decode(password).unwrap();
    let private_key = RsaPrivateKey::from_pkcs1_pem(RSA_KEY).unwrap();

    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &password).unwrap();
    String::from_utf8(decrypted).unwrap()
}

/// Hashes the given password and returns two things:
/// 1. The hashed password.
/// 2. The salt used to hash the password.
/// password: The password to hash.
pub fn hash_password(password: String) -> (String, String) {
    let salt = random::<[u8; 32]>();
    let salt = digest(&salt);
    let salted = format!("{}{}", password, salt);
    (hash(salted, DEFAULT_COST).unwrap(), salt)
}

/// Verifies the validity of the password.
/// given: The given password.
/// salt: The salt used to hash the password.
/// stored: The stored password.
pub fn verify_password(given:& String, salt: &String, stored: &String) -> bool {
    let salted = format!("{}{}", given, salt);
    verify(salted, &stored).unwrap()
}
