//! # Columnar Transposition Cipher
//!
//! ...
//!
//! TODO: implement without padding

use std::collections::HashMap;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("GERMAN");
///
/// let ciphertext = ciphers::columnar_transposition::cipher(plaintext, key);
/// assert_eq!(ciphertext, "NALCEHWTTDTTFSEELEEDSOAFEAHL")
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let mut key: Vec<u8> = key.bytes().collect();
    let plaintext = plaintext.as_bytes();
    let mut matrix: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

    // populate matrix
    for i in 0..plaintext.len() {
        matrix.entry(key[i % key.len()]).or_insert(vec![]);
        matrix
            .get_mut(&key[i % key.len()])
            .unwrap()
            .push(plaintext[i]);
    }

    key.sort();

    let mut ciphertext = vec![];
    for k in key.iter() {
        for byte in matrix.get(&k).unwrap() {
            ciphertext.push(*byte);
        }
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("NALCEHWTTDTTFSEELEEDSOAFEAHL");
/// let key = String::from("GERMAN");
///
/// let plaintext = ciphers::columnar_transposition::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    let key: Vec<u8> = key.bytes().collect();
    let ciphertext = ciphertext.as_bytes();
    let mut matrix: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

    let mut sorted_key = key.clone();
    sorted_key.sort();

    for k in sorted_key.iter() {
        matrix.insert(*k, vec![]);
    }

    // populate matrix
    let mut i = 0usize;
    let mut k = 0usize;
    while i < ciphertext.len() {
        // calculate length of current entry
        let col = key.iter().position(|&c| c == sorted_key[k]).unwrap();
        let len = if col < ciphertext.len() % key.len() {
            ciphertext.len() / key.len() + 1
        } else {
            ciphertext.len() / key.len()
        };

        for _ in 0..len {
            matrix.entry(sorted_key[k]).or_insert(vec![]);
            matrix.get_mut(&sorted_key[k]).unwrap().push(ciphertext[i]);
            i += 1;
        }

        k += 1;
    }

    let mut plaintext = vec![];
    for i in 0..ciphertext.len() {
        plaintext.push(matrix.get_mut(&key[i % key.len()]).unwrap().remove(0));
    }

    String::from_utf8(plaintext).unwrap()
}
