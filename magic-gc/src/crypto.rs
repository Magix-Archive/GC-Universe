use base64::{Engine, prelude::BASE64_STANDARD};
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt};

static RSA_KEY: &str = include_str!("../resources/private_key.pem");

/// RSA decrypts the password using RSA/ECB/PKCS1Padding.
/// password: The encrypted password in Base64.
pub fn decrypt_password(password: String) -> String {
    let password = BASE64_STANDARD.decode(password).unwrap();
    let private_key = RsaPrivateKey::from_pkcs1_pem(RSA_KEY).unwrap();

    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &password).unwrap();
    String::from_utf8(decrypted).unwrap()
}
