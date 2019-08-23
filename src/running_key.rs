//! # Running Key Cipher
//!
//! Implements the functionality for the Running Key cipher.
//!
//! The following is an excerpt from [Wikipedia](https://en.wikipedia.org/wiki/Running_key_cipher).
//! > In classical cryptography, the running key cipher is a type of polyalphabetic substitution
//! cipher in which a text, typically from a book, is used to provide a very long keystream.
//! Usually, the book to be used would be agreed ahead of time, while the passage to be used would
//! be chosen randomly for each message and secretly indicated somewhere in the message.

use crate::{Cipher, TABULA_RECTA};

/// `RunningKey` struct contains the key for the Running Key cipher, and implements the
/// functionality of the `Cipher` trait using the Running Key cipher method.
pub struct RunningKey {
    key: String,
}

impl RunningKey {
    /// `RunningKey` constructor takes the key for the Running Key cipher and returns a
    /// corresponding RunningKey struct.
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for RunningKey {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Running Key
    /// cipher and returns the ciphertext as a `String` object.
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

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Running Key
    /// cipher and returns the plaintext as a `String` object.
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
