//! # Simple Substitution Cipher
//!
//! Implements the functionality for the Simple Substitution cipher.
//!
//! The following is an excerpt from
//! [Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher#Simple_substitution).
//! > Substitution of single letters separately—simple substitution—can be demonstrated by writing
//! out the alphabet in some order to represent the substitution. This is termed a substitution
//! alphabet.
//!
//! > The cipher alphabet may be shifted or reversed (creating the Caesar and Atbash ciphers,
//! respectively) or scrambled in a more complex fashion, in which case it is called a mixed
//! alphabet or deranged alphabet. Traditionally, mixed alphabets may be created by first writing
//! out a keyword, removing repeated letters in it, then writing all the remaining letters in the
//! alphabet in the usual order.

use crate::{Cipher, CipherResult};

/// `Substitution` struct contains the key for the Simple Substitution cipher, and implements the
/// functionality of the `Cipher` trait using the Simple Substitution cipher method.
pub struct Substitution {
    key: String,
}

impl Substitution {
    /// `Substitution` constructor takes the key for the Simple Substitution cipher and returns a
    /// corresponding Substitution struct.
    pub fn new(key: &str) -> Self {
        assert_eq!(key.len(), 26);
        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for Substitution {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Simple
    /// Substitution cipher and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, Substitution};
    ///
    /// let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    ///
    /// let ctext = substitution.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ptext = ptext.to_ascii_uppercase();
        let key = self.key.as_bytes();

        let ctext = ptext.bytes().map(move |c| key[(c - 65) as usize]).collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Simple
    /// Substitution cipher and returns the plaintext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, Substitution};
    ///
    /// let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    ///
    /// let ptext = substitution.decipher("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        let ctext = ctext.to_ascii_uppercase();
        let ptext = ctext
            .bytes()
            .map(move |c| self.key.find(move |i| i == c as char).unwrap() as u8 + 65)
            .collect();

        Ok(String::from_utf8(ptext).unwrap())
    }
}
