//! # Autokey Cipher
//!
//! ...

use crate::TABULA_RECTA;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("FORTIFICATION");
///
/// let ciphertext = ciphers::autokey::cipher(plaintext, key);
/// assert_eq!(ciphertext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let key = key.as_bytes();
    let plaintext: Vec<u8> = plaintext.bytes().collect();

    let ciphertext: Vec<u8> = plaintext
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let y = match i {
                i if i < key.len() => key[i] as usize - 'A' as usize,
                _ => plaintext[i - key.len()] as usize - 'A' as usize,
            };

            TABULA_RECTA[y][*c as usize - 'A' as usize]
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
/// let key = String::from("FORTIFICATION");
///
/// let plaintext = ciphers::autokey::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    let key = key.as_bytes();
    let ciphertext: Vec<u8> = ciphertext.bytes().collect();

    let mut plaintext: Vec<u8> = Vec::with_capacity(ciphertext.len());
    for (i, c) in ciphertext.iter().enumerate() {
        let y = match i {
            i if i < key.len() => key[i] as usize - 'A' as usize,
            _ => plaintext[i - key.len()] as usize - 'A' as usize,
        };

        plaintext.push(
            TABULA_RECTA[y].iter().position(|&j| j == *c).unwrap() as u8 + 'A' as u8
        );
    }

    String::from_utf8(plaintext).unwrap()
}
