//! # Columnar Transposition Cipher
//!
//! ...

use std::collections::HashMap;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("GERMAN");
///
/// let ciphertext = ciphers::columnar_transposition::cipher(plaintext, key);
/// assert_eq!(ciphertext, "NALCXEHWTTDTTFSEELEEDSOAXFEAHL")
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let mut key: Vec<u8> = key.bytes().collect();
    let plaintext = plaintext.as_bytes();
    let mut matrix: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

    for k in key.iter() {
        matrix.insert(*k, vec![]);
    }

    let len = if plaintext.len() % key.len() == 0 {
        plaintext.len()
    } else {
        plaintext.len() + key.len() - plaintext.len() % key.len()
    };

    // populate matrix
    for i in 0..len {
        if i < plaintext.len() {
            matrix
                .get_mut(&key[i % key.len()])
                .unwrap()
                .push(plaintext[i]);
        } else {
            matrix.get_mut(&key[i % key.len()]).unwrap().push('X' as u8);
        }
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
/// let ciphertext = String::from("NALCXEHWTTDTTFSEELEEDSOAXFEAHL");
/// let key = String::from("GERMAN");
///
/// let plaintext = ciphers::columnar_transposition::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLEXX");
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

    for i in 0..ciphertext.len() {
        matrix
            .get_mut(&sorted_key[i / (ciphertext.len() / key.len())])
            .unwrap()
            .push(ciphertext[i]);
    }

    eprintln!("{:#?}", matrix);

    let mut plaintext = vec![];
    for i in 0..ciphertext.len() {
        plaintext.push(matrix.get(&key[i % key.len()]).unwrap()[i % ciphertext.len() / key.len()]);
    }

    String::from_utf8(plaintext).unwrap()
}
