//! # Atbash Cipher
//!
//! ...

use crate::Cipher;

/// `Atbash` struct ...
pub struct Atbash;

impl Atbash {
    /// `Atbash` constructor ...
    pub fn new() -> Self {
        Self {}
    }
}

impl Cipher for Atbash {
    /// `encipher` method ...
    /// 
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::atbash::Atbash;
    /// 
    /// let ptext = String::from("ATTACKATDAWN");
    /// let atbash = Atbash::new();
    ///
    /// let ctext = atbash.encipher(ptext);
    /// assert_eq!(ctext, "ZGGZXPZGWZDM");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let ctext = ptext
            .bytes()
            .map(|c| 90 - c + 65)
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    /// 
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::atbash::Atbash;
    /// 
    /// let ctext = String::from("ZGGZXPZGWZDM");
    /// let atbash = Atbash::new();
    ///
    /// let ptext = atbash.decipher(ctext);
    /// assert_eq!(ptext, "ATTACKATDAWN");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        self.encipher(ctext)
    }
}