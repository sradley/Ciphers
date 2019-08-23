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

use crate::{Cipher, CipherResult};

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

/// `Porta` struct contains the key for the Porta cipher, and implements the functionality of
/// the `Cipher` trait using the Porta cipher method.
pub struct Porta {
    key: String,
}

impl Porta {
    /// `Porta` constructor takes the key for the Porta cipher and returns a corresponding
    /// Porta struct.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_ascii_uppercase(),
        }
    }
}

impl Cipher for Porta {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Porta cipher
    /// and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, Porta};
    ///
    /// let porta = Porta::new("FORTIFICATION");
    ///
    /// let ctext = porta.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
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

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Porta cipher
    /// and returns the plaintext as a `String` object.
    /// 
    /// Note that the Porta cipher is reciprocal.
    ///
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
