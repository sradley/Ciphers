//! # Running Key Cipher
//!
//! Implements the functionality for the Running Key cipher.
//!
//! The following is an excerpt from [Wikipedia](https://en.wikipedia.org/wiki/Running_key_cipher).
//! > In classical cryptography, the running key cipher is a type of polyalphabetic substitution
//! cipher in which a text, typically from a book, is used to provide a very long keystream.
//! Usually, the book to be used would be agreed ahead of time, while the passage to be used would
//! be chosen randomly for each message and secretly indicated somewhere in the message.

use crate::{Cipher, CipherResult, TABULA_RECTA};

/// A Running Key cipher implementation.
pub struct RunningKey {
    key: String,
}

impl RunningKey {
    /// Takes the key for the Running Key cipher and returns a
    /// corresponding RunningKey struct.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for RunningKey {
    /// Enciphers the given plaintext (a str reference) using the Running Key
    /// cipher and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, RunningKey};
    ///
    /// let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    ///
    /// let ctext = running_key.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        assert!(self.key.len() >= ptext.len());
        let ptext = ptext.to_ascii_uppercase();

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

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Running Key
    /// cipher and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, RunningKey};
    ///
    /// let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    ///
    /// let ptext = running_key.decipher("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        assert!(self.key.len() >= ctext.len());
        let ctext = ctext.to_ascii_uppercase();

        let key = self.key.as_bytes();

        let ptext = ctext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i] as usize - 65;
                TABULA_RECTA[y].iter().position(|&j| j == c).unwrap() as u8 + 65
            })
            .collect();

        Ok(String::from_utf8(ptext).unwrap())
    }
}
