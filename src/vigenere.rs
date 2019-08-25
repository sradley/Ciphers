//! # Vigenere Cipher
//!
//! Implements the functionality for the Vigenere cipher.
//!
//! The following is an excerpt from
//! [Wikipedia](https://en.wikipedia.org/wiki/Vigenère_cipher).
//! > The Vigenère cipher is a method of encrypting alphabetic text by using a series of interwoven
//! Caesar ciphers, based on the letters of a keyword. It employs a form of polyalphabetic
//! substitution.
//!
//! > First described by Giovan Battista Bellaso in 1553, the cipher is easy to understand and
//! implement, but it resisted all attempts to break it until 1863, three centuries later. This
//! earned it the description le chiffre indéchiffrable (French for 'the indecipherable cipher')
//! Many people have tried to implement encryption schemes that are essentially Vigenère ciphers.
//! In 1863, Friedrich Kasiski was the first to publish a general method of deciphering Vigenère
//! ciphers.
//!
//! > In the 19th century the scheme was misattributed to Blaise de Vigenère (1523 – 1596), and so
//! acquired its present name.

use crate::{input, Cipher, CipherResult, TABULA_RECTA};

/// A Vigenere cipher implementation.
pub struct Vigenere {
    key: String,
}

impl Vigenere {
    /// Takes the key for the Vigenere cipher and returns a corresponding
    /// Vigenere struct.
    ///
    /// # Panics
    /// * If `key` is not alphabetic.
    pub fn new(key: &str) -> Self {
        input::is_alpha(key).expect("`key` must be alphabetic");

        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for Vigenere {
    /// Enciphers the given plaintext (a str reference) using the Vignere cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Vigenere};
    ///
    /// let vigenere = Vigenere::new("FORTIFICATION");
    ///
    /// let ctext = vigenere.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_alpha(ptext)?;

        let ptext = ptext.to_ascii_uppercase();
        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i % key.len()] as usize - 65;
                let x = c as usize - 65;

                TABULA_RECTA[y][x]
            })
            .collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Vigenere cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Vigenere};
    ///
    /// let vigenere = Vigenere::new("FORTIFICATION");
    ///
    /// let ptext = vigenere.decipher("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::is_alpha(ctext)?;

        let ctext = ctext.to_ascii_uppercase();
        let key = self.key.as_bytes();

        let ptext = ctext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = key[i % key.len()] as usize - 65;
                TABULA_RECTA[y].iter().position(|&j| j == c).unwrap() as u8 + 65
            })
            .collect();

        Ok(String::from_utf8(ptext).unwrap())
    }
}
