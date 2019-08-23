//! # Autokey Cipher
//!
//! Implements the functionality for the Autokey cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Autokey_cipher).
//! > An autokey cipher (also known as the autoclave cipher) is a cipher that incorporates the
//! message (the plaintext) into the key. The key is generated from the message in some automated
//! fashion, sometimes by selecting certain letters from the text or, more commonly, by adding a
//! short primer key to the front of the message.
//!
//! > There are two forms of autokey cipher: key-autokey and text-autokey ciphers. A key-autokey
//! cipher uses previous members of the keystream to determine the next element in the keystream. A
//! text-autokey uses the previous message text to determine the next element in the keystream.
//!
//! > In modern cryptography, self-synchronising stream ciphers are autokey ciphers.

use crate::{Cipher, CipherResult, TABULA_RECTA};

/// An Autokey cipher implementation.
pub struct Autokey {
    key: String,
}

impl Autokey {
    /// Takes the key for the Autokey cipher and returns a corresponding
    /// Autokey struct.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for Autokey {
    /// Enciphers the given plaintext (a str reference) using the Autokey cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Autokey};
    ///
    /// let autokey = Autokey::new("FORTIFICATION");
    ///
    /// let ctext = autokey.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ptext = ptext.to_ascii_uppercase();
        let key = self.key.as_bytes();
        let ptext: Vec<u8> = ptext.bytes().collect();

        let ctext: Vec<u8> = ptext
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let y = match i {
                    i if i < key.len() => key[i] as usize - 65,
                    _ => ptext[i - key.len()] as usize - 65,
                };

                TABULA_RECTA[y][*c as usize - 65]
            })
            .collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Autokey cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Autokey};
    ///
    /// let autokey = Autokey::new("FORTIFICATION");
    ///
    /// let ptext = autokey.decipher("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        let ctext = ctext.to_ascii_uppercase();
        let key = self.key.as_bytes();
        let ctext: Vec<u8> = ctext.bytes().collect();

        let mut ptext: Vec<u8> = Vec::with_capacity(ctext.len());
        for (i, c) in ctext.iter().enumerate() {
            let y = match i {
                i if i < key.len() => key[i] as usize - 65,
                _ => ptext[i - key.len()] as usize - 65,
            };

            ptext.push(TABULA_RECTA[y].iter().position(|&j| j == *c).unwrap() as u8 + 65);
        }

        Ok(String::from_utf8(ptext).unwrap())
    }
}
