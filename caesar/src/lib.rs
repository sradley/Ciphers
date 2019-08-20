//! # Caesar Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher_rot_1` test function ...
    #[test]
    fn cipher_rot_1() {
        let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");

        let ciphertext = crate::cipher(plaintext, 1);
        assert_eq!(ciphertext, "EFGFOE UIF FBTU XBMM PG UIF DBTUMF");
    }

    /// `test_rot_25` test function ...
    #[test]
    fn cipher_rot_25() {
        let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");

        let ciphertext = crate::cipher(plaintext, 25);
        assert_eq!(ciphertext, "CDEDMC SGD DZRS VZKK NE SGD BZRSKD");
    }

    /// `decipher_rot_1` test function ...
    #[test]
    fn decipher_rot_1() {
        let ciphertext = String::from("EFGFOE UIF FBTU XBMM PG UIF DBTUMF");

        let plaintext = crate::decipher(ciphertext, 1);
        assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
    }

    /// `decipher_rot_25` test function ...
    #[test]
    fn decipher_rot_25() {
        let ciphertext = String::from("CDEDMC SGD DZRS VZKK NE SGD BZRSKD");

        let plaintext = crate::decipher(ciphertext, 25);
        assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");
///
/// let ciphertext = caesar::cipher(plaintext, 1);
/// assert_eq!(ciphertext, "EFGFOE UIF FBTU XBMM PG UIF DBTUMF");
/// ```
/// 
/// ```
/// let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");
///
/// let ciphertext = caesar::cipher(plaintext, 25);
/// assert_eq!(ciphertext, "CDEDMC SGD DZRS VZKK NE SGD BZRSKD");
/// ```
pub fn cipher(plaintext: String, rotations: u8) -> String {
    let plaintext = plaintext
        .bytes()
        .map(move |c| match c {
            65u8...90u8 => (c + rotations - 'A' as u8) % 26 + 'A' as u8,
            _ => c,
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("EFGFOE UIF FBTU XBMM PG UIF DBTUMF");
///
/// let plaintext = caesar::decipher(ciphertext, 1);
/// assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
/// ```
/// 
/// ```
/// let ciphertext = String::from("CDEDMC SGD DZRS VZKK NE SGD BZRSKD");
///
/// let plaintext = caesar::decipher(ciphertext, 25);
/// assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
/// ```
pub fn decipher(ciphertext: String, rotations: u8) -> String {
    let ciphertext = ciphertext
        .bytes()
        .map(move |c| match c {
            65u8...90u8 => (c + (26 - rotations) - 'A' as u8) % 26 + 'A' as u8,
            _ => c,
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}
