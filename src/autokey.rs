//! # Autokey Cipher
//!
//! ...

use crate::{Cipher, TABULA_RECTA};

/// `Autokey` struct ...
pub struct Autokey {
    key: String,
}

impl Autokey {
    /// `Autokey` constructor ...
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for Autokey {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::autokey::Autokey;
    ///
    /// let autokey = Autokey::new("FORTIFICATION");
    ///
    /// let ctext = autokey.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let key = self.key.as_bytes();
        let ptext: Vec<u8> = ptext.bytes().collect();

        let ctext: Vec<u8> = ptext
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let y = match i {
                    i if i < key.len() => key[i] as usize - 65,
                    _ => ptext[i - key.len()] as usize - 65,
                };

                TABULA_RECTA[y][*c as usize - 65]
            })
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::autokey::Autokey;
    ///
    /// let autokey = Autokey::new("FORTIFICATION");
    ///
    /// let ptext = autokey.decipher("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let key = self.key.as_bytes();
        let ctext: Vec<u8> = ctext.bytes().collect();

        let mut ptext: Vec<u8> = Vec::with_capacity(ctext.len());
        for (i, c) in ctext.iter().enumerate() {
            let y = match i {
                i if i < key.len() => key[i] as usize - 65,
                _ => ptext[i - key.len()] as usize - 65,
            };

            ptext.push(TABULA_RECTA[y].iter().position(|&j| j == *c).unwrap() as u8 + 65);
        }

        String::from_utf8(ptext).unwrap()
    }
}
