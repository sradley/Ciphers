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
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Cipher for Autokey {
    /// `encipher` method ...
    /// 
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::autokey::Autokey;
    /// 
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let key = String::from("FORTIFICATION");
    /// let autokey = Autokey::new(key);
    ///
    /// let ctext = autokey.encipher(ptext);
    /// assert_eq!(ctext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let key = self.key.as_bytes();
        let ptext: Vec<u8> = ptext.bytes().collect();

        let ctext: Vec<u8> = ptext
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let y = match i {
                    i if i < key.len() => key[i] as usize - 'A' as usize,
                    _ => ptext[i - key.len()] as usize - 'A' as usize,
                };

                TABULA_RECTA[y][*c as usize - 'A' as usize]
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
    /// let ctext = String::from("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// let key = String::from("FORTIFICATION");
    /// let autokey = Autokey::new(key);
    ///
    /// let ptext = autokey.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let key = self.key.as_bytes();
        let ctext: Vec<u8> = ctext.bytes().collect();

        let mut ptext: Vec<u8> = Vec::with_capacity(ctext.len());
        for (i, c) in ctext.iter().enumerate() {
            let y = match i {
                i if i < key.len() => key[i] as usize - 'A' as usize,
                _ => ptext[i - key.len()] as usize - 'A' as usize,
            };

            ptext.push(TABULA_RECTA[y].iter().position(|&j| j == *c).unwrap() as u8 + 'A' as u8);
        }

        String::from_utf8(ptext).unwrap()
    }
}