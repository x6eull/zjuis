use num_bigint::BigUint;

//仅适用于长度不超62位且不含非ASCII字符的密码
pub fn encrypt(password: &str, exponent_hex: &str, modulus_hex: &str) -> String {
    let pwd_int = BigUint::from_bytes_be(password.as_bytes());
    let exp_int = BigUint::parse_bytes(exponent_hex.as_bytes(), 16).unwrap();
    let modulus_int = BigUint::parse_bytes(modulus_hex.as_bytes(), 16).unwrap();
    pwd_int.modpow(&exp_int, &modulus_int).to_str_radix(16)
}
