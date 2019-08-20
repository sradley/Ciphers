//! # Atbash Cipher
//!
//! ...

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("ATTACKATDAWN");
///
/// let ciphertext = ciphers::atbash::cipher(plaintext);
/// assert_eq!(ciphertext, "ZGGZXPZGWZDM");
/// ```
pub fn cipher(plaintext: String) -> String {
    let plaintext = plaintext
        .bytes()
        .map(|c| 'Z' as u8 - c + 'A' as u8)
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("ZGGZXPZGWZDM");
///
/// let plaintext = ciphers::atbash::decipher(ciphertext);
/// assert_eq!(plaintext, "ATTACKATDAWN");
/// ```
pub fn decipher(ciphertext: String) -> String {
    cipher(ciphertext)
}
