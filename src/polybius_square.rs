//! # Polybius Square Cipher
//!
//! Implements the functionality for the Polybius Square cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Polybius_square).
//! > In cryptography, the Polybius square, also known as the Polybius checkerboard, is a device
//! invented by the Ancient Greeks Cleoxenus and Democleitus, and perfected by the Ancient Greek
//! historian and scholar Polybius, for fractionating plaintext characters so that they can be
//! represented by a smaller set of symbols.
//!
//! TODO: handle unwraps (i.e. when trying to find a character that's not in the square)

use crate::{Cipher, CipherResult, CipherInputError, input};

/// A Polybius Square cipher implementation.
pub struct PolybiusSquare {
    key: String,
    chars: String,
}

impl PolybiusSquare {
    /// Takes the key and specified characters for the Polybius Square
    /// cipher and returns a corresponding PolybiusSquare struct.
    pub fn new(key: &str, chars: &str) -> Self {        
        input::is_ascii(key)
            .expect("`key` must be valid ascii");
        input::no_repeated_chars(key)
            .expect("`key` cannot contain repeated chars");
        input::is_ascii(chars)
            .expect("`chars` must be valid ascii");
        input::no_repeated_chars(chars)
            .expect("`chars` cannot contain repeated chars");

        if key.len() != chars.len() * chars.len() {
            panic!("`chars` must be of length sqrt(key.len())")
        }
        
        Self {
            key: String::from(key),
            chars: String::from(chars),
        }
    }
}

impl Cipher for PolybiusSquare {
    /// Enciphers the given plaintext (a str reference) using the Polybius Square
    /// cipher and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, PolybiusSquare};
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_ascii(ptext)?;
        input::in_alphabet(ptext, &self.key)?;

        let chars = self.chars.as_bytes();
        let mut ctext: Vec<u8> = Vec::with_capacity(ptext.len());

        for c in ptext.bytes() {
            let i = self.key.find(move |j| j == c as char).unwrap();

            ctext.push(chars[i / chars.len()]);
            ctext.push(chars[i % chars.len()]);
        }

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Polybius Square
    /// cipher and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, PolybiusSquare};
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ptext = ps.decipher("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::is_ascii(ctext)?;
        input::in_alphabet(ctext, &self.chars)?;
        if ctext.len() % 2 != 0 {
            return Err(CipherInputError::BadInput(
                String::from("`ctext` must contain an even number of chars")
            ))
        }

        let key = self.key.as_bytes();
        let ctext = ctext.as_bytes();
        let mut ptext: Vec<u8> = Vec::with_capacity(ctext.len());

        for i in (0..ctext.len()).step_by(2) {
            let y = self.chars.find(|c| c == ctext[i] as char).unwrap();
            let x = self.chars.find(|c| c == ctext[i + 1] as char).unwrap();

            ptext.push(key[y * self.chars.len() + x]);
        }

        Ok(String::from_utf8(ptext).unwrap())
    }
}
