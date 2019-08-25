//! # Affine Cipher
//!
//! Implements the functionality for the Affine cipher.
//!
//! The following is an excerpt from [Wikipedia](https://en.wikipedia.org/wiki/Affine_cipher).
//! > The affine cipher is a type of monoalphabetic substitution cipher, wherein each letter in an
//! alphabet is mapped to its numeric equivalent, encrypted using a simple mathematical function,
//! and converted back to a letter.
//!
//! > The formula used means that each letter encrypts to one other letter, and back again, meaning
//! the cipher is essentially a standard substitution cipher with a rule governing which letter goes
//! to which.
//!
//! > As such, it has the weaknesses of all substitution ciphers. Each letter is enciphered with the
//! function (ax + b) mod 26, where b is the magnitude of the shift.

use crate::{input, Cipher, CipherResult};

static RELATIVE_PRIMES: [i32; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];

/// An Affine cipher implementation.
pub struct Affine {
    a: i32,
    b: i32,
}

impl Affine {
    /// Takes the two keys for the Affine cipher and returns a
    /// corresponding Affine struct.
    pub fn new(a: i32, b: i32) -> Self {
        assert!(0 < a && a < 26, "`a` must be in the range [1, 26)");
        assert!(0 <= b && b < 26, "`b` must be in the range [0, 26)");
        assert!(
            RELATIVE_PRIMES.contains(&a),
            "`a` must be relatively prime to 26"
        );

        Self { a, b }
    }
}

impl Cipher for Affine {
    /// Enciphers the given plaintext (a str reference) using the Affine cipher
    /// and returns the ciphertext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Affine};
    ///
    /// let affine = Affine::new(7, 11);
    ///
    /// let ctext = affine.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext.unwrap(), "GNUNYGOINNLHOJLKKFUOINZLHOKN");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        input::is_alpha(ptext)?;

        let ptext = ptext.to_ascii_uppercase();
        let ctext = ptext
            .bytes()
            .map(move |c| ((self.a * (c as i32 - 65) + self.b) % 26) as u8 + 65)
            .collect();

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// Deciphers the given ciphertext (a str reference) using the Affine cipher
    /// and returns the plaintext as a `CipherResult`.
    ///
    /// # Example
    /// ```
    /// use ciphers::{Cipher, Affine};
    ///
    /// let affine = Affine::new(7, 11);
    ///
    /// let ptext = affine.decipher("GNUNYGOINNLHOJLKKFUOINZLHOKN");
    /// assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        input::is_alpha(ctext)?;

        let ctext = ctext.to_ascii_uppercase();
        let a_inv = invmod(self.a, 26).unwrap();

        let ptext = ctext
            .bytes()
            .map(move |c| (((a_inv * (c as i32 - 65 - self.b)) % 26 + 26) % 26) as u8 + 65)
            .collect();

        Ok(String::from_utf8(ptext).unwrap())
    }
}

fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
}

fn invmod(a: i32, m: i32) -> Option<i32> {
    let (g, x, _) = egcd(a, m);

    match g {
        1 => Some((x % m + m) % m),
        _ => None,
    }
}
