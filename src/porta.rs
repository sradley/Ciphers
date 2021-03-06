//! # Porta Cipher
//!
//! Implements the functionality for the Porta cipher.
//!
//! > The Porta Cipher is a polyalphabetic substitution cipher invented by Giovanni Battista della
//! Porta. Where the Vigenere cipher is a polyalphabetic cipher with 26 alphabets, the Porta is
//! basically the same except it only uses 13 alphabets.
//!
//! > The 13 cipher alphabets it uses are
//! reciprocal, so enciphering is the same as deciphering.

use crate::{input, Cipher, CipherResult};

static PORTA_TABLEU: [[u8; 13]; 13] = [
    [78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
    [79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78],
    [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79],
    [81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80],
    [82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81],
    [83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82],
    [84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83],
    [85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84],
    [86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85],
    [87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86],
    [88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87],
    [89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88],
    [90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89],
];

/// A Porta cipher implementation.
pub struct Porta {
    key: String,
}

impl Porta {
    /// Takes the key for the Porta cipher and returns a corresponding
    /// Porta struct.
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

impl Cipher for Porta {
    /// Enciphers the given plaintext (a str reference) using the Porta cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Porta};
    ///
    /// let porta = Porta::new("FORTIFICATION");
    ///
    /// let ctext = porta.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_alpha(ptext)?;

        let ptext = ptext.to_ascii_uppercase();
        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = (key[i % key.len()] as usize - 65) / 2;
                match c {
                    78...90 => PORTA_TABLEU[y].iter().position(|&j| j == c).unwrap() as u8 + 65,
                    _ => PORTA_TABLEU[y][c as usize - 65],
                }
            })
            .collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Porta cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// Note that the Porta cipher is reciprocal.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Porta};
    ///
    /// let porta = Porta::new("FORTIFICATION");
    ///
    /// let ptext = porta.decipher("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        self.encipher(ctext)
    }
}
