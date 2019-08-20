//! # Caesar Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher_rot_1` test function ...
    #[test]
    fn cipher_rot_1() {
        let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

        let ciphertext = crate::cipher(plaintext, 1);
        assert_eq!(ciphertext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    }

    /// `test_rot_25` test function ...
    #[test]
    fn cipher_rot_25() {
        let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

        let ciphertext = crate::cipher(plaintext, 25);
        assert_eq!(ciphertext, "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
    }

    /// `decipher_rot_1` test function ...
    #[test]
    fn decipher_rot_1() {
        let ciphertext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");

        let plaintext = crate::decipher(ciphertext, 1);
        assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
    }

    /// `decipher_rot_25` test function ...
    #[test]
    fn decipher_rot_25() {
        let ciphertext = String::from("CDEDMCSGDDZRSVZKKNESGDBZRSKD");

        let plaintext = crate::decipher(ciphertext, 25);
        assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
///
/// let ciphertext = caesar::cipher(plaintext, 1);
/// assert_eq!(ciphertext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
/// ```
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
///
/// let ciphertext = caesar::cipher(plaintext, 25);
/// assert_eq!(ciphertext, "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
/// ```
pub fn cipher(plaintext: String, rotations: u8) -> String {
    let plaintext = plaintext
        .bytes()
        .map(move |c| (c + rotations - 'A' as u8) % 26 + 'A' as u8)
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
///
/// let plaintext = caesar::decipher(ciphertext, 1);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
/// 
/// ```
/// let ciphertext = String::from("CDEDMCSGDDZRSVZKKNESGDBZRSKD");
///
/// let plaintext = caesar::decipher(ciphertext, 25);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, rotations: u8) -> String {
    let ciphertext = ciphertext
        .bytes()
        .map(move |c| (c + (26 - rotations) - 'A' as u8) % 26 + 'A' as u8)
        .collect();

    String::from_utf8(ciphertext).unwrap()
}
