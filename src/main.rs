
use std::sync::Arc;

use hasher::HasherKeccak; // https://crates.io/crates/hasher

use cita_trie::MemoryDB;
use cita_trie::{PatriciaTrie, Trie, verify_proof};

fn init_tree() -> PatriciaTrie<MemoryDB, HasherKeccak> {
    let memdb = Arc::new(MemoryDB::new(true));
    let hasher = Arc::new(HasherKeccak::new());

    let key = "test-key".as_bytes();
    let value = "test-value".as_bytes();

    let root = {
        let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));
        trie.insert(key.to_vec(), value.to_vec()).unwrap();

        let v = trie.get(key).unwrap();
        assert_eq!(Some(value.to_vec()), v);
        trie.root().unwrap()
    };

    PatriciaTrie::from(Arc::clone(&memdb), Arc::clone(&hasher), &root).unwrap()

}

fn main() {
    let key = "test-key".as_bytes();
    let mut trie = init_tree();
    let root = trie.root().unwrap();
    
    let merkle_proof = trie.get_proof(key).unwrap();
    let hasher = HasherKeccak::new();
    assert!(verify_proof(&root, key, merkle_proof.clone(), hasher).unwrap().is_some());

    let (prove_hash, verify_hash) = guest::build_hash();
    let (hash_output, hash_proof) = prove_hash(key.to_vec());
    let is_hash_valid = verify_hash(hash_proof);

    println!("hash proof valid: {}", is_hash_valid);
    println!("hash output: {:?}", hash_output);

    let (prove_trie, verify_trie) = guest::build_trie();
    let (result, proof) = prove_trie(root, key.to_vec(), merkle_proof);
    let is_valid = verify_trie(proof);

    println!("result valid: {}", result.is_some());
    println!("proof valid: {}", is_valid);


}