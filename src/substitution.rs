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
    pub fn new(key: String) -> Self {
        assert_eq!(key.len(), 26);
        Self { key }
    }
}

impl Cipher for Substitution {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::substitution::Substitution;
    ///
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    /// let substitution = Substitution::new(key);
    ///
    /// let ctext = substitution.encipher(ptext);
    /// assert_eq!(ctext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// ```
    fn encipher(&self, ptext: String) -> String {
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
    /// let ctext = String::from("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    /// let substitution = Substitution::new(key);
    ///
    /// let ptext = substitution.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let ptext = ctext
            .bytes()
            .map(move |c| self.key.find(move |i| i == c as char).unwrap() as u8 + 65)
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}
