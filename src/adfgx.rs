//! # ADFGX Cipher
//!
//! Implements the functionality for the ADFGX cipher.
//!
//! > The ADFGX cipher was a field cipher used by the German Army during World War I. It is closely
//! related to the ADFGXC cipher (i.e. ADFGX is the predecessor to ADFGVX).
//!
//! > The cipher was a fractionating transposition cipher which combined a modified Polybius square
//! with a single columnar transposition.
//!
//! > The cipher is named after the five possible letters used in the ciphertext: A, D, F, G and X.
//! The letters were chosen deliberately because they are very different from one another in the
//! Morse code. That reduced the possibility of operator error.

use crate::columnar_transposition::ColumnarTransposition;
use crate::polybius_square::PolybiusSquare;
use crate::Cipher;

/// `ADFGX` struct contains the key and keyword for the ADFGX cipher, and implements the
/// functionality of the `Cipher` trait using the ADFGX cipher method.
pub struct ADFGX {
    key: String,
    keyword: String,
}

impl ADFGX {
    /// `ADFGX` constructor takes the key and keyword for the ADFGX cipher and returns a
    /// corresponding ADFGX struct.
    pub fn new(key: &str, keyword: &str) -> Self {
        assert_eq!(key.len(), 25);
        Self {
            key: String::from(key),
            keyword: String::from(keyword),
        }
    }
}

impl Cipher for ADFGX {
    /// `encipher` method enciphers the given plaintext (a str reference) using the ADFGX cipher
    /// and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgx::ADFGX;
    ///
    /// let adfgx = ADFGX::new("PHQGMEAYNOFDXKRCVSZWBUTIL", "GERMAN");
    ///
    /// let ctext = adfgx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let ps = PolybiusSquare::new(&self.key, "ADFGX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ct.encipher(&ps.encipher(ptext))
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the ADFGX cipher
    /// and returns the plaintext as a `String` object.
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgx::ADFGX;
    ///
    /// let adfgx = ADFGX::new("PHQGMEAYNOFDXKRCVSZWBUTIL", "GERMAN");
    ///
    /// let ptext = adfgx.decipher("FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let ps = PolybiusSquare::new(&self.key, "ADFGX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ps.decipher(&ct.decipher(ctext))
    }
}
