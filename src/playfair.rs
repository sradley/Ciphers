//! # Playfair Cipher
//!
//! ...
//!
//! TODO: handle unwraps (i.e. when trying to find a character that's not in the square)

use crate::Cipher;

/// `Playfair` struct ...
pub struct Playfair {
    key: String,
}

impl Playfair {
    /// `Playfair` constructor ...
    pub fn new(key: &str) -> Self {
        assert_eq!(key.len(), 25);
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for Playfair {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::playfair::Playfair;
    ///
    /// let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    ///
    /// let ctext = playfair.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let mut ptext: Vec<u8> = ptext.bytes().collect();
        if ptext.len() % 2 != 0 {
            ptext.push(88);
        }

        let key = self.key.as_bytes();

        let mut ctext = Vec::with_capacity(ptext.len());
        for i in (0..ptext.len()).step_by(2) {
            if ptext[i] == ptext[i + 1] {
                ptext[i + 1] = 88;
            }

            let yx1 = key.iter().position(|&c| c == ptext[i]).unwrap();
            let yx2 = key.iter().position(|&c| c == ptext[i + 1]).unwrap();

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

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::playfair::Playfair;
    ///
    /// let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    ///
    /// let ptext = playfair.decipher("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    /// assert_eq!(ptext, "DEFENDTHEXASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        let ctext = ctext.as_bytes();
        let key = self.key.as_bytes();

        let mut ptext = Vec::with_capacity(ctext.len());
        for i in (0..ctext.len()).step_by(2) {
            let yx1 = key.iter().position(|&c| c == ctext[i]).unwrap();
            let yx2 = key.iter().position(|&c| c == ctext[i + 1]).unwrap();

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

        String::from_utf8(ptext).unwrap()
    }
}
