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
            let y = c as usize - 'A' as usize;
            let x = TABULA_RECTA[y]
                .iter()
                .position(|&j| j == key[i % key.len()])
                .unwrap();

            TABULA_RECTA[0][x]
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
