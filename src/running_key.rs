//! # Running Key Cipher
//! 
//! ...

use crate::TABULA_RECTA;

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
///
/// let ciphertext = ciphers::running_key::cipher(plaintext, key);
/// assert_eq!(ciphertext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    assert!(key.len() >= plaintext.len());

    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .enumerate()
        .map(move |(i, c)| {
            let y = key[i] as usize - 'A' as usize;
            let x = c as usize - 'A' as usize;

            TABULA_RECTA[y][x] as u8
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
/// let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
///
/// let plaintext = ciphers::running_key::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    assert!(key.len() >= ciphertext.len());

    let key = key.as_bytes();

    let plaintext = ciphertext
        .bytes()
        .enumerate()
        .map(move |(i, c)| {
            let y = key[i] as usize - 'A' as usize;

            let mut x = 0u8;
            for j in 0..TABULA_RECTA[y].len() {
                if TABULA_RECTA[y][j] as u8 == c {
                    x = j as u8;
                    break;
                }
            }

            x + 'A' as u8
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}
