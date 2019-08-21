//! # ADFGVX Cipher
//!
//! ...

use crate::{columnar_transposition, polybius_square};

/// `cipher` function ...
///
/// ```
/// use ciphers::adfgvx;
///
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
/// let keyword = String::from("GERMAN");
///
/// let ciphertext = adfgvx::cipher(plaintext, key, keyword);
/// assert_eq!(ciphertext, "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
/// ```
pub fn cipher(plaintext: String, key: String, keyword: String) -> String {
    assert_eq!(key.len(), 36);
    columnar_transposition::cipher(
        polybius_square::cipher(plaintext, key, String::from("ADFGVX")),
        keyword,
    )
}

/// `decipher` function ...
///
/// ```
/// use ciphers::adfgvx;
///
/// let ciphertext = String::from("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
/// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
/// let keyword = String::from("GERMAN");
///
/// let plaintext = adfgvx::decipher(ciphertext, key, keyword);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String, keyword: String) -> String {
    assert_eq!(key.len(), 36);
    polybius_square::decipher(
        columnar_transposition::decipher(ciphertext, keyword),
        key,
        String::from("ADFGVX"),
    )
}
