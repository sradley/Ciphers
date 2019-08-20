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
            if i < key.len() {
                TABULA_RECTA[key[i] as usize - 'A' as usize][*c as usize - 'A' as usize] as u8
            } else {
                TABULA_RECTA[plaintext[i - key.len()] as usize - 'A' as usize]
                    [*c as usize - 'A' as usize] as u8
            }
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
        if i < key.len() {
            let y = key[i] as usize - 'A' as usize;
            let mut x = 0u8;
            for j in 0..TABULA_RECTA[y].len() {
                if *c == TABULA_RECTA[y][j] as u8 {
                    x = j as u8;
                    break;
                }
            }

            plaintext.push(x + 'A' as u8);
        } else {
            let y = plaintext[i - key.len()] as usize - 'A' as usize;
            let mut x = 0u8;
            for j in 0..TABULA_RECTA[y].len() {
                if *c == TABULA_RECTA[y][j] as u8 {
                    x = j as u8;
                    break;
                }
            }

            plaintext.push(x + 'A' as u8);
        }
    }

    String::from_utf8(plaintext).unwrap()
}
