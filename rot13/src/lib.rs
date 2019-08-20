//! # ROT13 Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("ATTACK AT DAWN");

        let ciphertext = crate::cipher(plaintext);
        assert_eq!(ciphertext, "NGGNPX NG QNJA");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("NGGNPX NG QNJA");

        let plaintext = crate::decipher(ciphertext);
        assert_eq!(plaintext, "ATTACK AT DAWN");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("ATTACK AT DAWN");
///
/// let ciphertext = rot13::cipher(plaintext);
/// assert_eq!(ciphertext, "NGGNPX NG QNJA");
/// ```
pub fn cipher(plaintext: String) -> String {
    let plaintext = plaintext
        .bytes()
        .map(|c| match c {
            65u8...90u8 => (c + 13 - 'A' as u8) % 26 + 'A' as u8,
            _ => c,
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("NGGNPX NG QNJA");
///
/// let plaintext = rot13::decipher(ciphertext);
/// assert_eq!(plaintext, "ATTACK AT DAWN");
/// ```
pub fn decipher(ciphertext: String) -> String {
    cipher(ciphertext)
}
