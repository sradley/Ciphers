//! # Polybius Square Cipher
//!
//! ...
//!
//! TODO: handle unwraps (i.e. when trying to find a character that's not in the square)

use crate::Cipher;

/// `PolybiusSquare` struct ...
pub struct PolybiusSquare {
    key: String,
    chars: String,
}

impl PolybiusSquare {
    /// `PolybiusSquare` constructor ...
    pub fn new(key: &str, chars: &str) -> Self {
        assert_eq!(key.len(), chars.len() * chars.len());
        Self {
            key: String::from(key),
            chars: String::from(chars),
        }
    }
}

impl Cipher for PolybiusSquare {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::polybius_square::PolybiusSquare;
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let chars = self.chars.as_bytes();
        let mut ctext: Vec<u8> = Vec::with_capacity(ptext.len());

        for c in ptext.bytes() {
            let i = self.key.find(move |j| j == c as char).unwrap();

            ctext.push(chars[i / chars.len()]);
            ctext.push(chars[i % chars.len()]);
        }

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::polybius_square::PolybiusSquare;
    ///
    /// let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");
    ///
    /// let ptext = ps.decipher("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        assert_eq!(ctext.len() % 2, 0);

        let key = self.key.as_bytes();
        let ctext = ctext.as_bytes();
        let mut ptext: Vec<u8> = Vec::with_capacity(ctext.len());

        for i in (0..ctext.len()).step_by(2) {
            let y = self.chars.find(|c| c == ctext[i] as char).unwrap();
            let x = self.chars.find(|c| c == ctext[i + 1] as char).unwrap();

            ptext.push(key[y * self.chars.len() + x]);
        }

        String::from_utf8(ptext).unwrap()
    }
}
