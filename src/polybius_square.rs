//! # Polybius Square Cipher
//!
//! ...
//! 
//! TODO: handle unwraps (i.e. when trying to find a character that's not in the square)

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
/// let chars = String::from("ABCDE");
///
/// let ciphertext = ciphers::polybius_square::cipher(plaintext, key, chars);
/// assert_eq!(
///      ciphertext,
///     "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
/// );
/// ```
pub fn cipher(plaintext: String, key: String, chars: String) -> String {
    assert_eq!(key.len(), chars.len() * chars.len());

    let chars = chars.as_bytes();
    let mut ciphertext: Vec<u8> = Vec::with_capacity(plaintext.len());

    for c in plaintext.bytes() {
        let i = key.find(move |j| j == c as char).unwrap();

        ciphertext.push(chars[i / chars.len()]);
        ciphertext.push(chars[i % chars.len()]);
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// let ciphertext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
/// let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
/// let chars = String::from("ABCDE");
///
/// let plaintext = ciphers::polybius_square::decipher(ciphertext, key, chars);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String, chars: String) -> String {
    assert_eq!(key.len(), chars.len() * chars.len());
    assert_eq!(ciphertext.len() % 2, 0);

    let key = key.as_bytes();
    let ciphertext = ciphertext.as_bytes();
    let mut plaintext: Vec<u8> = Vec::with_capacity(ciphertext.len());

    for i in (0..ciphertext.len()).step_by(2) {
        let y = chars.find(|c| c == ciphertext[i] as char).unwrap();
        let x = chars.find(|c| c == ciphertext[i + 1] as char).unwrap();

        plaintext.push(key[y * chars.len() + x]);
    }

    String::from_utf8(plaintext).unwrap()
}
