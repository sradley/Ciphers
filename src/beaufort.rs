//! # Beaufort Cipher
//!
//! ...

use crate::{Cipher, TABULA_RECTA};

/// `Beaufort` struct ...
pub struct Beaufort {
    key: String,
}

impl Beaufort {
    /// `Beaufort` constructor ...
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Cipher for Beaufort {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::beaufort::Beaufort;
    ///
    /// let key = String::from("FORTIFICATION");
    /// let beaufort = Beaufort::new(key);
    /// 
    /// let ctext = beaufort.encipher(String::from("DEFENDTHEEASTWALLOFTHECASTLE"));
    /// assert_eq!(ctext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(|(i, c)| {
                let y = c as usize - 65;
                let x = TABULA_RECTA[y]
                    .iter()
                    .position(|&j| j == key[i % key.len()])
                    .unwrap();

                TABULA_RECTA[0][x]
            })
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::beaufort::Beaufort;
    ///
    /// let key = String::from("FORTIFICATION");
    /// let beaufort = Beaufort::new(key);
    ///
    /// let ptext = beaufort.decipher(String::from("CKMPVCPVWPIWUJOGIUAPVWRIWUUK"));
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        self.encipher(ctext)
    }
}
