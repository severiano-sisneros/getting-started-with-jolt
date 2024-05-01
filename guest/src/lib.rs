#![no_main]
use hasher::{Hasher, HasherKeccak}; // https://crates.io/crates/hasher
use cita_trie::verify_proof;

#[jolt::provable(stack_size = 10000, max_input_size = 128)]
fn trie(
    root_hash: Vec<u8>,
    key: Vec<u8>,
    proof: Vec<Vec<u8>>,
) -> Option<Vec<u8>> {
    let hasher = HasherKeccak::new();
    let result = verify_proof(&root_hash, &key, proof, hasher);
    if result.is_ok() {
        return result.unwrap();
    } else {
        return None;
    }
}

#[jolt::provable]
fn hash(data: Vec<u8>) -> Vec<u8> {
    let hasher = HasherKeccak::new();
    hasher.digest(&data)
}

#[jolt::provable]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}
