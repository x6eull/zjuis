use num_bigint::BigUint;
use reqwest::{Client, Error};
use serde::Deserialize;

/// 使用RSA算法对密码进行加密。仅适用于长度不超62位，且只含ASCII字符的密码。
///
/// password为实际密码，exponent_hex为RSA公钥的指数，modulus_hex为RSA公钥的模数。
/// 后两者均为16进制字符串。
pub fn encrypt(password: &str, exponent_hex: &str, modulus_hex: &str) -> String {
    let pwd_int = BigUint::from_bytes_be(password.as_bytes());
    let exp_int = BigUint::parse_bytes(exponent_hex.as_bytes(), 16).unwrap();
    let modulus_int = BigUint::parse_bytes(modulus_hex.as_bytes(), 16).unwrap();
    pwd_int.modpow(&exp_int, &modulus_int).to_str_radix(16)
}

#[derive(Debug, Deserialize)]
pub struct PubKey {
    pub modulus: String,
    pub exponent: String,
}

pub async fn get_public_key(client: &Client) -> Result<PubKey, Error> {
    client
        .get("https://zjuam.zju.edu.cn/cas/v2/getPubKey")
        .send()
        .await?
        .json::<PubKey>()
        .await
}
