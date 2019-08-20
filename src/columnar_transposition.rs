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
    let plaintext = plaintext.as_bytes();
    let mut key: Vec<u8> = key.bytes().collect();
    let mut key_map: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

    for k in key.iter() {
        key_map.insert(*k, vec![]);
    }

    for i in (0..plaintext.len()).step_by(key.len()) {
        for j in 0..key.len() {
            if i + j >= plaintext.len() {
                // insert 'X'
                key_map.get_mut(&key[j]).unwrap().push(88u8);
            } else {
                // default
                key_map.get_mut(&key[j]).unwrap().push(plaintext[i + j]);
            }
        }
    }

    // sort alphabetically
    key.sort();

    let mut ciphertext: Vec<u8> = Vec::with_capacity(plaintext.len());
    for k in key.iter() {
        for byte in key_map.get(&k).unwrap() {
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
    let ciphertext = ciphertext.as_bytes();
    let key: Vec<u8> = key.bytes().collect();
    let mut key_map: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

    for k in key.iter() {
        key_map.insert(*k, vec![]);
    }

    // sort key alphabetically
    let mut sorted_key = key.clone();
    sorted_key.sort();

    for i in 0..key.len() {
        for j in 0..(ciphertext.len() / key.len()) {
            key_map
                .get_mut(&sorted_key[i])
                .unwrap()
                .push(ciphertext[i * (key.len() - 1) + j]);
        }
    }

    let mut plaintext: Vec<u8> = Vec::with_capacity(ciphertext.len());
    for i in 0..(ciphertext.len() / key.len()) {
        for k in key.iter() {
            plaintext.push(key_map.get(&k).unwrap()[i]);
        }
    }

    String::from_utf8(plaintext).unwrap()
}
