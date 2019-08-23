//! # Four-Square Cipher
//!
//! Implements the functionality for the Four-Square cipher.
//!
//! The following excerpt is from [Wikipedia](https://en.wikipedia.org/wiki/Four-square_cipher).
//! > The four-square cipher is a manual symmetric encryption technique. It was invented by the
//! famous French cryptographer Felix Delastelle.
//!
//! > The technique encrypts pairs of letters (digraphs), and thus falls into a category of ciphers
//! known as polygraphic substitution ciphers. This adds significant strength to the encryption when
//! compared with monographic substitution ciphers which operate on single characters. The use of
//! digraphs makes the four-square technique less susceptible to frequency analysis attacks, as the
//! analysis must be done on 676 possible digraphs rather than just 26 for monographic substitution.
//!
//! > The frequency analysis of digraphs is possible, but considerably more difficult - and it
//! generally requires a much larger ciphertext in order to be useful.
//!
//! TODO: handle unwraps (i.e. when trying to find a letter that's not in the alphabet)

use crate::{Cipher, CipherResult};

static ALPHABET: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90,
];

/// `FourSquare` struct contains the two keys for the Four-Square cipher, and implements the
/// functionality of the `Cipher` trait using the Four-Square cipher method.
pub struct FourSquare {
    key1: String,
    key2: String,
}

impl FourSquare {
    /// `FourSquare` constructor constructor takes the two keys for the Four-Square cipher and
    /// returns a corresponding FourSquare struct.
    pub fn new(key1: &str, key2: &str) -> Self {
        assert_eq!(key1.len(), 25);
        assert_eq!(key2.len(), 25);
        Self {
            key1: key1.to_ascii_uppercase(),
            key2: key2.to_ascii_uppercase(),
        }
    }
}

impl Cipher for FourSquare {
    /// `encipher` method enciphers the given plaintext (a str reference) using the Four-Square
    /// cipher and returns the ciphertext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, FourSquare};
    ///
    /// let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");
    ///
    /// let ctext = four_square.encipher("ATTACKATDAWN");
    /// assert_eq!(ctext.unwrap(), "TIYBFHTIZBSY");
    /// ```
    fn encipher(&self, ptext: &str) -> CipherResult {
        let ptext = ptext.to_ascii_uppercase();
        let mut ptext: Vec<u8> = ptext.bytes().collect();
        if ptext.len() % 2 != 0 {
            ptext.push(88);
        }

        let key1 = self.key1.as_bytes();
        let key2 = self.key2.as_bytes();

        let mut ctext = Vec::with_capacity(ptext.len());
        for i in (0..ptext.len()).step_by(2) {
            let yx1 = ALPHABET.iter().position(|&c| c == ptext[i]).unwrap();
            let yx2 = ALPHABET.iter().position(|&c| c == ptext[i + 1]).unwrap();

            let (y1, x1) = (yx1 / 5, yx1 % 5);
            let (y2, x2) = (yx2 / 5, yx2 % 5);

            ctext.push(key1[y1 * 5 + x2]);
            ctext.push(key2[y2 * 5 + x1]);
        }

        Ok(String::from_utf8(ctext).unwrap())
    }

    /// `decipher` method deciphers the given ciphertext (a str reference) using the Four-Square
    /// cipher and returns the plaintext as a `String` object.
    ///
    /// ```
    /// use ciphers::{Cipher, FourSquare};
    ///
    /// let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");
    ///
    /// let ptext = four_square.decipher("TIYBFHTIZBSY");
    /// assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
    /// ```
    fn decipher(&self, ctext: &str) -> CipherResult {
        assert_eq!(ctext.len() % 2, 0);
        let ctext = ctext.to_ascii_uppercase();
        let ctext = ctext.as_bytes();

        let mut ptext = Vec::with_capacity(ctext.len());
        for i in (0..ctext.len()).step_by(2) {
            let yx1 = self.key1.find(|c| c == ctext[i] as char).unwrap();
            let yx2 = self.key2.find(|c| c == ctext[i + 1] as char).unwrap();

            let (y1, x2) = (yx1 / 5, yx1 % 5);
            let (y2, x1) = (yx2 / 5, yx2 % 5);

            ptext.push(ALPHABET[y1 * 5 + x1]);
            ptext.push(ALPHABET[y2 * 5 + x2]);
        }

        Ok(String::from_utf8(ptext).unwrap())
    }
}
