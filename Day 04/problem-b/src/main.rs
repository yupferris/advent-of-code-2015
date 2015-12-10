extern crate crypto;

use crypto::digest::Digest;
use crypto::md5;

type Hash = [u8; 16];

fn main() {
    let number = (1..).find(|x| compute_hash_string("iwrupvqb".to_string(), *x).starts_with("000000")).unwrap();
    println!("The number is: {}", number);
}

fn compute_hash_string(key: String, number: i32) -> String {
    let hash = compute_hash(key, number);
    hash.iter().fold(String::new(), |acc, x| format!("{}{:>02x}", acc, x))
}

fn compute_hash(key: String, number: i32) -> Hash {
    let mut digest = md5::Md5::new();
    digest.input_str(&format!("{}{}", key, number));
    let mut result = [0; 16];
    digest.result(&mut result);
    result
}
