//! # Substitution Cipher
//!
//! ...

use crate::Cipher;

/// `Substitution` struct ...
pub struct Substitution {
    key: String,
}

impl Substitution {
    /// `Substitution` constructor ...
    pub fn new(key: &str) -> Self {
        assert_eq!(key.len(), 26);
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for Substitution {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::substitution::Substitution;
    ///
    /// let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    ///
    /// let ctext = substitution.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let key = self.key.as_bytes();

        let ctext = ptext.bytes().map(move |c| key[(c - 65) as usize]).collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::substitution::Substitution;
    ///
    /// let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    ///
    /// let ptext = substitution.decipher("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let ptext = ctext
            .bytes()
            .map(move |c| self.key.find(move |i| i == c as char).unwrap() as u8 + 65)
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}
