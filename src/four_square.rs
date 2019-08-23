//! # Four-Square Cipher
//!
//! ...
//!
//! TODO: handle unwraps (i.e. when trying to find a letter that's not in the alphabet)

use crate::Cipher;

static ALPHABET: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90,
];

/// `FourSquare` struct ...
pub struct FourSquare {
    key1: String,
    key2: String,
}

impl FourSquare {
    /// `FourSquare` constructor ...
    pub fn new(key1: String, key2: String) -> Self {
        assert_eq!(key1.len(), 25);
        assert_eq!(key2.len(), 25);
        Self { key1, key2 }
    }
}

impl Cipher for FourSquare {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::four_square::FourSquare;
    ///
    /// let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    /// let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
    /// let four_square = FourSquare::new(key1, key2);
    ///
    /// let ctext = four_square.encipher(String::from("ATTACKATDAWN"));
    /// assert_eq!(ctext, "TIYBFHTIZBSY");
    /// ```
    fn encipher(&self, ptext: String) -> String {
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

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::four_square::FourSquare;
    ///
    /// let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    /// let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
    /// let four_square = FourSquare::new(key1, key2);
    ///
    /// let ptext = four_square.decipher( String::from("TIYBFHTIZBSY"));
    /// assert_eq!(ptext, "ATTACKATDAWN");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        assert_eq!(ctext.len() % 2, 0);
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

        String::from_utf8(ptext).unwrap()
    }
}
