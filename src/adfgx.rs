//! # ADFGX Cipher
//!
//! ...

use crate::{columnar_transposition, polybius_square};

/// `cipher` function ...
///
/// ```
/// use ciphers::adfgx;
///
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
/// let keyword = String::from("GERMAN");
///
/// let ciphertext = adfgx::cipher(plaintext, key, keyword);
/// assert_eq!(ciphertext, "FFDGDDADXXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFXADAFDXDDXXDDADGXXGXX");
/// ```
pub fn cipher(plaintext: String, key: String, keyword: String) -> String {
    let step1 = polybius_square::cipher(plaintext, key, String::from("ADFGX"));
    let step2 = columnar_transposition::cipher(step1, keyword);

    step2
}

/// `cipher` function ...
///
/// ```
/// use ciphers::adfgx;
///
/// let ciphertext = String::from("FFDGDDADXXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFXADAFDXDDXXDDADGXXGXX");
/// let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
/// let keyword = String::from("GERMAN");
///
/// let plaintext = adfgx::decipher(ciphertext, key, keyword);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLELL");
/// ```
pub fn decipher(ciphertext: String, key: String, keyword: String) -> String {
    polybius_square::decipher(
        columnar_transposition::decipher(ciphertext, keyword),
        key,
        String::from("ADFGX"),
    )
}
