//! # Beaufort Cipher
//!
//! Implements the functionality for the Beaufort cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Beaufort_cipher).
//! > The Beaufort cipher, created by Sir Francis Beaufort, is a substitution cipher similar to the
//! Vigenère cipher, with a slightly modified enciphering mechanism and tableau. Its most famous
//! application was in a rotor-based cipher machine, the Hagelin M-209.
//!
//! > The Beaufort cipher is based on the Beaufort square which is essentially the same as a
//! Vigenère square but in reverse order starting with the letter "Z" in the first row, where the
//! first row and the last column serve the same purpose.

use crate::{Cipher, CipherResult, TABULA_RECTA};

/// `Beaufort` struct contains the key for the Beaufort cipher, and implements the functionality of
/// the `Cipher` trait using the Beaufort cipher method.
pub struct Beaufort {
    key: String,
}

impl Beaufort {
    /// `Beaufort` constructor takes the key for the Beaufort cipher and returns a corresponding
    /// Beaufort struct.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for Beaufort {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Beaufort cipher
    /// and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, Beaufort};
    ///
    /// let beaufort = Beaufort::new("FORTIFICATION");
    ///
    /// let ctext = beaufort.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ptext = ptext.to_ascii_uppercase();
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

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Beaufort cipher
    /// and returns the plaintext as a `String` object.
    ///
    /// Note that the Beaufort cipher is reciprocal.
    ///
    /// ```
    /// use ciphers::{Cipher, Beaufort};
    ///
    /// let beaufort = Beaufort::new("FORTIFICATION");
    ///
    /// let ptext = beaufort.decipher("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        let ctext = ctext.to_ascii_uppercase();
        self.encipher(&ctext)
    }
}
