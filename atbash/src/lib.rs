//! # Atbash Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("ATTACK AT DAWN");

        let ciphertext = crate::cipher(plaintext);
        assert_eq!(ciphertext, "ZGGZXP ZG WZDM");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("ZGGZXP ZG WZDM");

        let plaintext = crate::decipher(ciphertext);
        assert_eq!(plaintext, "ATTACK AT DAWN");
    }
}

/// `cipher` function ...
pub fn cipher(plaintext: String) -> String {
    let plaintext: Vec<u8> = plaintext
        .bytes()
        .map(move |c| match c {
            65u8...90u8 => 'Z' as u8 - c + 'A' as u8,
            _ => c,
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
pub fn decipher(ciphertext: String) -> String {
    cipher(ciphertext)
}
