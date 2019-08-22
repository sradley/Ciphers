//! # ADFGVX Cipher
//!
//! ...

use crate::Cipher;
use crate::columnar_transposition::ColumnarTransposition;
use crate::polybius_square::PolybiusSquare;

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
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    /// let keyword = String::from("GERMAN");
    /// let adfgvx = ADFGVX::new(key, keyword);
    ///
    /// let ctext = adfgvx.encipher(ptext);
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
    /// let ctext = String::from("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    /// let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    /// let keyword = String::from("GERMAN");
    /// let adfgvx = ADFGVX::new(key, keyword);
    ///
    /// let ptext = adfgvx.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let ps = PolybiusSquare::new(self.key.clone(), String::from("ADFGVX"));
        let ct = ColumnarTransposition::new(self.keyword.clone());

        ps.decipher(ct.decipher(ctext))
    }
}
