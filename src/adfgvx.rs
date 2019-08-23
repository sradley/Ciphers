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
    pub fn new(key: String, keyword: String) -> Self {
        assert_eq!(key.len(), 36);
        Self { key, keyword }
    }
}

impl Cipher for ADFGVX {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgvx::ADFGVX;
    ///
    /// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    /// let keyword = String::from("GERMAN");
    /// let adfgvx = ADFGVX::new(key, keyword);
    ///
    /// let ctext = adfgvx.encipher(String::from("DEFENDTHEEASTWALLOFTHECASTLE"));
    /// assert_eq!(ctext, "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let ps = PolybiusSquare::new(self.key.clone(), String::from("ADFGVX"));
        let ct = ColumnarTransposition::new(self.keyword.clone());

        ct.encipher(ps.encipher(ptext))
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::adfgvx::ADFGVX;
    ///
    /// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    /// let keyword = String::from("GERMAN");
    /// let adfgvx = ADFGVX::new(key, keyword);
    ///
    /// let ptext = adfgvx.decipher(String::from("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"));
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let ps = PolybiusSquare::new(self.key.clone(), String::from("ADFGVX"));
        let ct = ColumnarTransposition::new(self.keyword.clone());

        ps.decipher(ct.decipher(ctext))
    }
}
