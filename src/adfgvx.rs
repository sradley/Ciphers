//! # ADFGVX Cipher
//!
//! Implements the functionality for the ADFGVX cipher.
//!
//! The following is an excerpt from [Wikipedia](https://en.wikipedia.org/wiki/ADFGVX_cipher).
//! > In cryptography, the ADFGVX cipher was a field cipher used by the German Army on the Western
//! Front during World War I. ADFGVX was in fact an extension of an earlier cipher called ADFGX.
//!
//! > Invented by Lieutenant Fritz Nebel (1891–1977) and introduced in March 1918, the cipher was a
//! fractionating transposition cipher which combined a modified Polybius square with a single
//! columnar transposition.W
//!
//! > The cipher is named after the six possible letters used in the ciphertext: A, D, F, G, V and
//! X. The letters were chosen deliberately because they are very different from one another in the
//! Morse code. That reduced the possibility of operator error.

use crate::{Cipher, CipherResult, ColumnarTransposition, PolybiusSquare};

/// An ADFGVX cipher implementation.
pub struct ADFGVX {
    key: String,
    keyword: String,
}

impl ADFGVX {
    /// Takes the key and keyword for the ADFGVX cipher and returns a
    /// corresponding ADFGVX struct.
    ///
    /// # Panics
    /// * If `key` is not 36 chars in length.
    /// * If `key` contains repeated chars.
    /// * If `key` is not valid ascii.
    /// * If `keyword` is not valid ascii.
    pub fn new(key: &str, keyword: &str) -> Self {
        assert_eq!(key.len(), 36, "`key` must be 36 chars in length");

        Self {
            key: String::from(key),
            keyword: String::from(keyword),
        }
    }
}

impl Cipher for ADFGVX {
    /// Enciphers the given plaintext (a str reference) using the ADFGVX cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, ADFGVX};
    ///
    /// let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");
    ///
    /// let ctext = adfgvx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ps = PolybiusSquare::new(&self.key, "ADFGVX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ct.encipher(&ps.encipher(ptext)?)
    }

    /// Deciphers the given ciphertext (a str reference) using the ADFGVX cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, ADFGVX};
    ///
    /// let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");
    ///
    /// let ptext = adfgvx.decipher("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        let ps = PolybiusSquare::new(&self.key, "ADFGVX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ps.decipher(&ct.decipher(ctext)?)
    }
}
