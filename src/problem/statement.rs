use crate::snark::proof::MODULUS;
use cryptography::exercises::{ec_point::Field, finite_field::Fp};
use std::env;

pub static TARGET_BYTE_LENGTH: usize = 16;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let n = decode_arg(&args[1]);
    let s = decode_arg(&args[2]);

    let nullifier = Fp::new(n, MODULUS).unwrap();
    let secret = Fp::new(s, MODULUS).unwrap();
    let k = Fp::new(3, MODULUS).unwrap();

    let hash = nullifier.pow(2) + k * secret;
    println!("hash: 0x{}, {}", hex::encode(hash.num.to_be_bytes()), hash);
}

fn decode_arg(s: &String) -> i64 {
    if s.starts_with("0x") {
        let bytes: [u8; 8] = hex::decode(normalize(&s[2..])).unwrap().try_into().unwrap();
        i64::from_be_bytes(bytes)
    } else {
        s.parse::<i64>().unwrap()
    }
}

fn normalize(s: &str) -> String {
    if s.len() < TARGET_BYTE_LENGTH {
        return format!("{}{}", "0".repeat(TARGET_BYTE_LENGTH - s.len()), s);
    }

    s[s.len() - TARGET_BYTE_LENGTH..].into()
}
