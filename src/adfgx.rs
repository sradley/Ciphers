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
    pub fn new(key: String, keyword: String) -> Self {
        assert_eq!(key.len(), 25);
        Self { key, keyword }
    }
}

impl Cipher for ADFGX {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgx::ADFGX;
    ///
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    /// let keyword = String::from("GERMAN");
    /// let adfgx = ADFGX::new(key, keyword);
    ///
    /// let ctext = adfgx.encipher(ptext);
    /// assert_eq!(ctext, "FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let ps = PolybiusSquare::new(self.key.clone(), String::from("ADFGX"));
        let ct = ColumnarTransposition::new(self.keyword.clone());

        ct.encipher(ps.encipher(ptext))
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgx::ADFGX;
    ///
    /// let ctext = String::from("FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    /// let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    /// let keyword = String::from("GERMAN");
    /// let adfgx = ADFGX::new(key, keyword);
    ///
    /// let ptext = adfgx.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let ps = PolybiusSquare::new(self.key.clone(), String::from("ADFGX"));
        let ct = ColumnarTransposition::new(self.keyword.clone());

        ps.decipher(ct.decipher(ctext))
    }
}
