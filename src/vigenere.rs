//! # Vigenere Cipher
//!
//! ...

use crate::{Cipher, TABULA_RECTA};

/// `Vigenere` struct ...
pub struct Vigenere {
    key: String,
}

impl Vigenere {
    /// `Vigenere` constructor ...
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Cipher for Vigenere {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::vigenere::Vigenere;
    ///
    /// let key = String::from("FORTIFICATION");
    /// let vigenere = Vigenere::new(key);
    ///
    /// let ctext = vigenere.encipher(String::from("DEFENDTHEEASTWALLOFTHECASTLE"));
    /// assert_eq!(ctext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i % key.len()] as usize - 65;
                let x = c as usize - 65;

                TABULA_RECTA[y][x]
            })
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::vigenere::Vigenere;
    ///
    /// let key = String::from("FORTIFICATION");
    /// let vigenere = Vigenere::new(key);
    ///
    /// let ptext = vigenere.decipher(String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS"));
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let key = self.key.as_bytes();

        let ptext = ctext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i % key.len()] as usize - 65;
                TABULA_RECTA[y].iter().position(|&j| j == c).unwrap() as u8 + 65
            })
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}
