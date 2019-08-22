//! # Caesar Cipher
//!
//! ...

use crate::Cipher;

/// `Caesar` struct ...
pub struct Caesar {
    key: u8,
}

impl Caesar {
    /// `Caesar` constructor ...
    pub fn new(key: u8) -> Self {
        Self { key }
    }
}

impl Cipher for Caesar {
    /// `encipher` method ...
    /// 
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::caesar::Caesar;
    /// 
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let caesar = Caesar::new(1);
    ///
    /// let ctext = caesar.encipher(ptext);
    /// assert_eq!(ctext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let ctext = ptext
            .bytes()
            .map(move |c| (c + self.key - 65) % 26 + 65)
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    /// 
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::caesar::Caesar;
    /// 
    /// let ctext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    /// let caesar = Caesar::new(1);
    ///
    /// let ptext = caesar.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let ptext = ctext
            .bytes()
            .map(move |c| (c + (26 - self.key) - 65) % 26 + 65)
            .collect();

        String::from_utf8(ptext).unwrap()
    }
}