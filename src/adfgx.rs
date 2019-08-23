//! # ADFGX Cipher
//!
//! ...

use crate::columnar_transposition::ColumnarTransposition;
use crate::polybius_square::PolybiusSquare;
use crate::Cipher;

/// `ADFGX` struct ...
pub struct ADFGX {
    key: String,
    keyword: String,
}

impl ADFGX {
    /// `ADFGX` constructor ...
    pub fn new(key: &str, keyword: &str) -> Self {
        assert_eq!(key.len(), 25);
        Self {
            key: String::from(key),
            keyword: String::from(keyword),
        }
    }
}

impl Cipher for ADFGX {
    /// `encipher` method ...
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

    /// `decipher` method ...
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
