//! # Substitution Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");
        let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

        let ciphertext = crate::cipher(plaintext, key);
        assert_eq!(ciphertext, "GIUIFG CEI IPRC TPNN DU CEI QPRCNI");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("GIUIFG CEI IPRC TPNN DU CEI QPRCNI");
        let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

        let plaintext = crate::decipher(ciphertext, key);
        assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFEND THE EAST WALL OF THE CASTLE");
/// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
///
/// let ciphertext = substitution::cipher(plaintext, key);
/// assert_eq!(ciphertext, "GIUIFG CEI IPRC TPNN DU CEI QPRCNI");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    assert_eq!(key.len(), 26);

    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .map(move |c| match c {
            65u8...90u8 => key[(c - 'A' as u8) as usize],
            _ => c,
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("GIUIFG CEI IPRC TPNN DU CEI QPRCNI");
/// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
///
/// let plaintext = substitution::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFEND THE EAST WALL OF THE CASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    assert_eq!(key.len(), 26);

    let plaintext = ciphertext
        .bytes()
        .map(move |c| match c {
            65u8...90u8 => key.find(move |i: char| i == c as char).unwrap() as u8 + 'A' as u8,
            _ => c,
        })
        .collect();

    String::from_utf8(plaintext).unwrap()
}
