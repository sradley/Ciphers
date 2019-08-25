//! # Playfair Cipher
//!
//! Implements the functionality for the Playfair cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Playfair_cipher).
//! > The Playfair cipher or Playfair square or Wheatstone-Playfair cipher is a manual symmetric
//! encryption technique and was the first literal digram substitution cipher. The scheme was
//! invented in 1854 by Charles Wheatstone, but bears the name of Lord Playfair for promoting its
//! use.
//!
//! > The technique encrypts pairs of letters (bigrams or digrams), instead of single letters as in
//! the simple substitution cipher and rather more complex Vigenère cipher systems then in use. The
//! Playfair is thus significantly harder to break since the frequency analysis used for simple
//! substitution ciphers does not work with it.
//!
//! > The frequency analysis of bigrams is possible, but considerably more difficult. With 600
//! possible bigrams rather than the 26 possible monograms (single symbols, usually letters in this
//! context), a considerably larger cipher text is required in order to be useful.

use crate::{input, Cipher, CipherInputError, CipherResult};

/// A Playfair cipher implementation
pub struct Playfair {
    key: String,
    pad: u8,
}

impl Playfair {
    /// Takes the key for the Playfair cipher and returns a corresponding
    /// Playfair struct.
    ///
    /// # Panics
    /// * If `key` is not 25 chars in length.
    /// * If `key` is not valid ascii.
    /// * If `key` contains repeated characters.
    /// * If `pad` is not contained within `key`.
    pub fn new(key: &str, pad: char) -> Self {
        assert_eq!(key.len(), 25, "`key` must be 25 chars in length");
        input::is_ascii(key).expect("`key` must be valid ascii");
        input::no_repeated_chars(key).expect("`key` cannot contain repeated chars");

        assert_ne!(key.find(pad), None, "`key` must contain `pad`");

        Self {
            key: String::from(key),
            pad: pad as u8,
        }
    }
}

impl Cipher for Playfair {
    /// Enciphers the given plaintext (a str reference) using the Playfair cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Playfair};
    ///
    /// let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", 'X');
    ///
    /// let ctext = playfair.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::in_alphabet(ptext, &self.key)?;

        let mut ptext: Vec<u8> = ptext.bytes().collect();
        if ptext.len() % 2 != 0 {
            ptext.push(self.pad);
        }

        let key = self.key.as_bytes();

        let mut ctext = Vec::with_capacity(ptext.len());
        for i in (0..ptext.len()).step_by(2) {
            if ptext[i] == ptext[i + 1] {
                ptext[i + 1] = self.pad;
            }

            let yx1 = match key.iter().position(|&c| c == ptext[i]) {
                Some(val) => val,
                None => return Err(CipherInputError::NotInAlphabet),
            };
            let yx2 = match key.iter().position(|&c| c == ptext[i + 1]) {
                Some(val) => val,
                None => return Err(CipherInputError::NotInAlphabet),
            };

            let (y1, x1) = (yx1 / 5, yx1 % 5);
            let (y2, x2) = (yx2 / 5, yx2 % 5);

            if y1 != y2 && x1 != x2 {
                // different rows and columns
                ctext.push(key[y1 * 5 + x2]);
                ctext.push(key[y2 * 5 + x1]);
            } else if y1 == y2 {
                // same row
                ctext.push(key[y1 * 5 + (x1 + 1) % 5]);
                ctext.push(key[y2 * 5 + (x2 + 1) % 5]);
            } else if x1 == x2 {
                // same column
                ctext.push(key[(y1 + 1) % 5 * 5 + x1]);
                ctext.push(key[(y2 + 1) % 5 * 5 + x2]);
            }
        }

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Playfair cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Playfair};
    ///
    /// let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", 'X');
    ///
    /// let ptext = playfair.decipher("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEXASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::in_alphabet(ctext, &self.key)?;
        if ctext.len() % 2 != 0 {
            return Err(CipherInputError::BadInput(String::from(
                "`ctext` must contain an even number of chars",
            )));
        }

        let ctext = ctext.as_bytes();
        let key = self.key.as_bytes();

        let mut ptext = Vec::with_capacity(ctext.len());
        for i in (0..ctext.len()).step_by(2) {
            let yx1 = match key.iter().position(|&c| c == ctext[i]) {
                Some(val) => val,
                None => return Err(CipherInputError::NotInAlphabet),
            };
            let yx2 = match key.iter().position(|&c| c == ctext[i + 1]) {
                Some(val) => val,
                None => return Err(CipherInputError::NotInAlphabet),
            };

            let (y1, x1) = (yx1 / 5, yx1 % 5);
            let (y2, x2) = (yx2 / 5, yx2 % 5);

            if y1 != y2 && x1 != x2 {
                // different rows and columns
                ptext.push(key[y1 * 5 + x2]);
                ptext.push(key[y2 * 5 + x1]);
            } else if y1 == y2 {
                // same row
                ptext.push(key[y1 * 5 + (x1 + 5 - 1) % 5]);
                ptext.push(key[y2 * 5 + (x2 + 5 - 1) % 5]);
            } else if x1 == x2 {
                // same column
                ptext.push(key[(y1 + 5 - 1) % 5 * 5 + x1]);
                ptext.push(key[(y2 + 5 - 1) % 5 * 5 + x2]);
            }
        }

        Ok(String::from_utf8(ptext).unwrap())
    }
}
