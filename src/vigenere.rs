//! # Vigenere Cipher
//!
//! ...

use crate::TABULA_RECTA;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("FORTIFICATION");
///
/// let ciphertext = ciphers::vigenere::cipher(plaintext, key);
/// assert_eq!(ciphertext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .enumerate()
        .map(move |(i, c)| {
            let y = key[i % key.len()] as usize - 'A' as usize;
            let x = c as usize - 'A' as usize;

            TABULA_RECTA[y][x]
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
/// let key = String::from("FORTIFICATION");
///
/// let plaintext = ciphers::vigenere::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    let key = key.as_bytes();

    let plaintext = ciphertext
        .bytes()
        .enumerate()
        .map(move |(i, c)| {
            let y = key[i % key.len()] as usize - 'A' as usize;
            TABULA_RECTA[y].iter().position(|&j| j == c).unwrap() as u8 + 'A' as u8
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}
