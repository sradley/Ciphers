//! # Porta Cipher
//!
//! ...

use crate::Cipher;

static PORTA_TABLEU: [[u8; 13]; 13] = [
    [78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
    [79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78],
    [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79],
    [81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80],
    [82, 83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81],
    [83, 84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82],
    [84, 85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83],
    [85, 86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84],
    [86, 87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85],
    [87, 88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86],
    [88, 89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87],
    [89, 90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88],
    [90, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89],
];

/// `Porta` struct ...
pub struct Porta {
    key: String,
}

impl Porta {
    /// `Porta` constructor
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

impl Cipher for Porta {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::porta::Porta;
    ///
    /// let porta = Porta::new("FORTIFICATION");
    ///
    /// let ctext = porta.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
        let key = self.key.as_bytes();

        let ctext = ptext
            .bytes()
            .enumerate()
            .map(move |(i, c)| {
                let y = (key[i % key.len()] as usize - 65) / 2;
                match c {
                    78...90 => PORTA_TABLEU[y].iter().position(|&j| j == c).unwrap() as u8 + 65,
                    _ => PORTA_TABLEU[y][c as usize - 65],
                }
            })
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::porta::Porta;
    ///
    /// let porta = Porta::new("FORTIFICATION");
    ///
    /// let ptext = porta.decipher("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
        self.encipher(ctext)
    }
}
