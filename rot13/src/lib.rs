//! # ROT13 Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("ATTACKATDAWN");

        let ciphertext = crate::cipher(plaintext);
        assert_eq!(ciphertext, "NGGNPXNGQNJA");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("NGGNPXNGQNJA");

        let plaintext = crate::decipher(ciphertext);
        assert_eq!(plaintext, "ATTACKATDAWN");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("ATTACKATDAWN");
///
/// let ciphertext = rot13::cipher(plaintext);
/// assert_eq!(ciphertext, "NGGNPXNGQNJA");
/// ```
pub fn cipher(plaintext: String) -> String {
    let plaintext = plaintext
        .bytes()
        .map(|c| (c + 13 - 'A' as u8) % 26 + 'A' as u8)
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("NGGNPXNGQNJA");
///
/// let plaintext = rot13::decipher(ciphertext);
/// assert_eq!(plaintext, "ATTACKATDAWN");
/// ```
pub fn decipher(ciphertext: String) -> String {
    cipher(ciphertext)
}
