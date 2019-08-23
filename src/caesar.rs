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

use crate::Cipher;

/// `Caesar` struct contains the key for the Caesar cipher, and implements the functionality of
/// the `Cipher` trait using the Caesar cipher method.
pub struct Caesar {
    key: u8,
}

impl Caesar {
    /// `Caesar` constructor takes the key for the Caesar cipher and returns a corresponding
    /// Caesar struct.
    pub fn new(key: u8) -> Self {
        Self { key }
    }
}

impl Cipher for Caesar {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Caesar cipher
    /// and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::caesar::Caesar;
    ///
    /// let caesar = Caesar::new(1);
    ///
    /// let ctext = caesar.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let ctext = ptext
            .bytes()
            .map(move |c| (c + self.key - 65) % 26 + 65)
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Caesar cipher
    /// and returns the plaintext as a `String` object.
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::caesar::Caesar;
    ///
    /// let caesar = Caesar::new(1);
    ///
    /// let ptext = caesar.decipher("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let ptext = ctext
            .bytes()
            .map(move |c| (c + (26 - self.key) - 65) % 26 + 65)
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}
