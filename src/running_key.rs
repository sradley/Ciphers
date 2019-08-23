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
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for RunningKey {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::running_key::RunningKey;
    ///
    /// let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    ///
    /// let ctext = running_key.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
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
    /// let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    ///
    /// let ptext = running_key.decipher("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
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
