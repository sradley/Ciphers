//! # Caesar Cipher
//!
//! Implements the functionality for the Caesar cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher).
//! > In cryptography, a Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's
//! code or Caesar shift, is one of the simplest and most widely known encryption techniques. It is
//! a type of substitution cipher in which each letter in the plaintext is replaced by a letter some
//! fixed number of positions down the alphabet. For example, with a left shift of 3, D would be
//! replaced by A, E would become B, and so on. The method is named after Julius Caesar, who used it
//! in his private correspondence.
//!
//! > The encryption step performed by a Caesar cipher is often incorporated as part of more complex
//! schemes, such as the Vigenère cipher, and still has modern application in the ROT13 system. As
//! with all single-alphabet substitution ciphers, the Caesar cipher is easily broken and in modern
//! practice offers essentially no communications security.

use crate::{Cipher, CipherResult, input};

/// A Caesar cipher implementation.
pub struct Caesar {
    key: u8,
}

impl Caesar {
    /// Takes the key for the Caesar cipher and returns a corresponding
    /// Caesar struct.
    pub fn new(key: u8) -> Self {
        Self { key }
    }
}

impl Cipher for Caesar {
    /// Enciphers the given plaintext (a str reference) using the Caesar cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Caesar};
    ///
    /// let caesar = Caesar::new(1);
    ///
    /// let ctext = caesar.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_alpha(ptext)?;

        let ptext = ptext.to_ascii_uppercase();
        let ctext = ptext
            .bytes()
            .map(move |c| (c + self.key - 65) % 26 + 65)
            .collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Caesar cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Caesar};
    ///
    /// let caesar = Caesar::new(1);
    ///
    /// let ptext = caesar.decipher("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::is_alpha(ctext)?;

        let ctext = ctext.to_ascii_uppercase();
        let ptext = ctext
            .bytes()
            .map(move |c| (c + (26 - self.key) - 65) % 26 + 65)
            .collect();

        Ok(String::from_utf8(ptext).unwrap())
    }
}
