//! # Running Key Cipher
//!
//! ...

use crate::{Cipher, TABULA_RECTA};

/// `RunningKey` struct ...
pub struct RunningKey {
    key: String,
}

impl RunningKey {
    /// `RunningKey` constructor ...
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Cipher for RunningKey {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::running_key::RunningKey;
    ///
    /// let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    /// let running_key = RunningKey::new(key);
    ///
    /// let ctext = running_key.encipher(String::from("DEFENDTHEEASTWALLOFTHECASTLE"));
    /// assert_eq!(ctext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        assert!(self.key.len() >= ptext.len());

        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i] as usize - 65;
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
    /// use ciphers::running_key::RunningKey;
    ///
    /// let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    /// let running_key = RunningKey::new(key);
    ///
    /// let ptext = running_key.decipher(String::from("KSBHBHLALIDMVGKYZKYAHXUAAWGM"));
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        assert!(self.key.len() >= ctext.len());

        let key = self.key.as_bytes();

        let ptext = ctext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i] as usize - 65;
                TABULA_RECTA[y].iter().position(|&j| j == c).unwrap() as u8 + 65
            })
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}
