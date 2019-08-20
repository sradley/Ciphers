//! # Substitution Cipher
//!
//! ...

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
///
/// let ciphertext = ciphers::substitution::cipher(plaintext, key);
/// assert_eq!(ciphertext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    assert_eq!(key.len(), 26);

    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .map(move |c| key[(c - 'A' as u8) as usize])
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
/// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
///
/// let plaintext = ciphers::substitution::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    assert_eq!(key.len(), 26);

    let plaintext = ciphertext
        .bytes()
        .map(move |c| key.find(move |i| i == c as char).unwrap() as u8 + 'A' as u8)
        .collect();

    String::from_utf8(plaintext).unwrap()
}