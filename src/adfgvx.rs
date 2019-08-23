//! # ADFGVX Cipher
//!
//! ...

use crate::columnar_transposition::ColumnarTransposition;
use crate::polybius_square::PolybiusSquare;
use crate::Cipher;

/// `ADFGVX` struct ...
pub struct ADFGVX {
    key: String,
    keyword: String,
}

impl ADFGVX {
    /// `ADFGVX` constructor ...
    pub fn new(key: &str, keyword: &str) -> Self {
        assert_eq!(key.len(), 36);
        Self {
            key: String::from(key),
            keyword: String::from(keyword),
        }
    }
}

impl Cipher for ADFGVX {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgvx::ADFGVX;
    ///
    /// let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");
    ///
    /// let ctext = adfgvx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let ps = PolybiusSquare::new(&self.key, "ADFGVX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ct.encipher(&ps.encipher(ptext))
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgvx::ADFGVX;
    ///
    /// let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");
    ///
    /// let ptext = adfgvx.decipher("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let ps = PolybiusSquare::new(&self.key, "ADFGVX");
        let ct = ColumnarTransposition::new(&self.keyword);

        ps.decipher(&ct.decipher(ctext))
    }
}
