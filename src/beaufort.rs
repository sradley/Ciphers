//! # Beaufort Cipher
//! 
//! ...

use crate::TABULA_RECTA;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("FORTIFICATION");
///
/// let ciphertext = ciphers::beaufort::cipher(plaintext, key);
/// assert_eq!(ciphertext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .enumerate()
        .map(|(i, c)| {
            let mut x = 0usize;
            for j in 0..TABULA_RECTA[c as usize - 'A' as usize].len() {
                if TABULA_RECTA[c as usize - 'A' as usize][j] as u8 == key[i % key.len()] {
                    x = j;
                    break;
                }
            }

            TABULA_RECTA[0][x] as u8
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
/// let key = String::from("FORTIFICATION");
///
/// let plaintext = ciphers::beaufort::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    cipher(ciphertext, key)
}