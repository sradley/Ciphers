//! # Polybius Square Cipher
//!
//! ...

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
///
/// let ciphertext = ciphers::polybius_square::cipher(plaintext, key);
/// assert_eq!(
///      ciphertext,
///     "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
/// );
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    assert_eq!(key.len(), 25);

    let key = key.as_bytes();
    let mut ciphertext: Vec<u8> = Vec::with_capacity(plaintext.len());

    for c in plaintext.bytes() {
        let mut i = 0u8;
        for j in 0..key.len() {
            if key[j] == c {
                i = j as u8;
                break;
            }
        }

        ciphertext.push((i / 5) + 'A' as u8);
        ciphertext.push((i % 5) + 'A' as u8);
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
/// let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
///
/// let plaintext = ciphers::polybius_square::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    assert_eq!(key.len(), 25);
    assert_eq!(ciphertext.len() % 2, 0);

    let key = key.as_bytes();
    let ciphertext = ciphertext.as_bytes();
    let mut plaintext: Vec<u8> = Vec::with_capacity(ciphertext.len());

    for i in (0..ciphertext.len()).step_by(2) {
        let y = ciphertext[i] - 'A' as u8;
        let x = ciphertext[i + 1] - 'A' as u8;

        plaintext.push(key[(y * 5 + x) as usize]);
    }

    String::from_utf8(plaintext).unwrap()
}