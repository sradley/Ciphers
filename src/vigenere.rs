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
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for Vigenere {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::vigenere::Vigenere;
    ///
    /// let vigenere = Vigenere::new("FORTIFICATION");
    ///
    /// let ctext = vigenere.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
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
    /// let vigenere = Vigenere::new("FORTIFICATION");
    ///
    /// let ptext = vigenere.decipher("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
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
