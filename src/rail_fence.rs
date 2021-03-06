//! # Rail-fence Cipher
//!
//! Implements the functionality for the Rail-fence cipher.
//!
//! The following is an excerpt from [Wikipedia](https://en.wikipedia.org/wiki/Rail_fence_cipher).
//! > In the rail fence cipher, the plain text is written downwards and diagonally on successive
//! "rails" of an imaginary fence, then moving up when the bottom rail is reached. When the top rail
//! is reached, the message is written downwards again until the whole plaintext is written out. The
//! message is then read off in rows.

use crate::{input, Cipher, CipherResult};

/// A Rail-fence cipher implementation.
pub struct RailFence {
    key: usize,
}

impl RailFence {
    /// Takes the key for the Rail-fence cipher and returns a corresponding
    /// RailFence struct.
    ///
    /// # Panics
    /// * If `key` is less than or equal to 0.
    pub fn new(key: usize) -> Self {
        assert!(key > 0, "`key` must be greater than 0");

        Self { key }
    }
}

impl Cipher for RailFence {
    /// Enciphers the given plaintext (a str reference) using the Rail-fence
    /// cipher and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, RailFence};
    ///
    /// let rail_fence = RailFence::new(4);
    ///
    /// let ctext = rail_fence.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "DTTFSEDHSWOTATFNEAALHCLEELEE");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_ascii(ptext)?;

        let mut ctext = Vec::with_capacity(ptext.len());
        let ptext: Vec<u8> = ptext.bytes().collect();

        let mut line = 0usize;
        while line < self.key - 1 {
            let skip = 2 * (self.key - line - 1);
            let mut j = 0usize;

            let mut i = line;
            while i < ptext.len() {
                ctext.push(*ptext.get(i).unwrap());

                if line == 0 || j % 2 == 0 {
                    i += skip;
                } else {
                    i += 2 * (self.key - 1) - skip;
                }

                j += 1;
            }

            line += 1;
        }

        for i in (line..ptext.len()).step_by(2 * (self.key - 1)) {
            ctext.push(*ptext.get(i).unwrap());
        }

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Rail-fence
    /// cipher and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, RailFence};
    ///
    /// let rail_fence = RailFence::new(4);
    ///
    /// let ptext = rail_fence.decipher("DTTFSEDHSWOTATFNEAALHCLEELEE");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::is_ascii(ctext)?;

        let mut ptext = vec![0u8; ctext.len()];
        let ctext: Vec<u8> = ctext.bytes().collect();
        let mut k = 0usize;

        let mut line = 0usize;
        while line < self.key - 1 {
            let skip = 2 * (self.key - line - 1);
            let mut j = 0usize;

            let mut i = line;
            while i < ctext.len() {
                ptext[i] = *ctext.get(k).unwrap();
                k += 1;

                if line == 0 || j % 2 == 0 {
                    i += skip;
                } else {
                    i += 2 * (self.key - 1) - skip;
                }

                j += 1;
            }

            line += 1;
        }

        for i in (line..ctext.len()).step_by(2 * (self.key - 1)) {
            ptext[i] = *ctext.get(k).unwrap();
            k += 1;
        }

        Ok(String::from_utf8(ptext).unwrap())
    }
}
