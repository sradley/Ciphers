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

use crate::{Cipher, CipherResult};

/// `PolybiusSquare` struct contains the key and specified characters for the Polybius Square
/// cipher, and implements the functionality of the `Cipher` trait using the Polybius Square cipher
/// method.
pub struct PolybiusSquare {
    key: String,
    chars: String,
}

impl PolybiusSquare {
    /// `PolybiusSquare` constructor takes the key and specified characters for the Polybius Square
    /// cipher and returns a corresponding PolybiusSquare struct.
    pub fn new(key: &str, chars: &str) -> Self {
        assert_eq!(key.len(), chars.len() * chars.len());
        Self {
            key: key.to_ascii_uppercase(),
            chars: chars.to_ascii_uppercase(),
        }
    }
}

impl Cipher for PolybiusSquare {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Polybius Square
    /// cipher and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, PolybiusSquare};
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ptext = ptext.to_ascii_uppercase();
        let chars = self.chars.as_bytes();
        let mut ctext: Vec<u8> = Vec::with_capacity(ptext.len());

        for c in ptext.bytes() {
            let i = self.key.find(move |j| j == c as char).unwrap();

            ctext.push(chars[i / chars.len()]);
            ctext.push(chars[i % chars.len()]);
        }

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Polybius Square
    /// cipher and returns the plaintext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, PolybiusSquare};
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ptext = ps.decipher("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        assert_eq!(ctext.len() % 2, 0);
        let ctext = ctext.to_ascii_uppercase();

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
