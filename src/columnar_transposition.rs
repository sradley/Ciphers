//! # Columnar Transposition Cipher
//!
//! ...

use crate::Cipher;
use std::collections::HashMap;

/// `ColumnarTransposition` struct ...
pub struct ColumnarTransposition {
    key: String,
}

impl ColumnarTransposition {
    /// `ColumnarTransposition` constructor ...
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Cipher for ColumnarTransposition {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::columnar_transposition::ColumnarTransposition;
    ///
    /// let key = String::from("GERMAN");
    /// let ct = ColumnarTransposition::new(key);
    ///
    /// let ctext = ct.encipher(String::from("DEFENDTHEEASTWALLOFTHECASTLE"));
    /// assert_eq!(ctext, "NALCEHWTTDTTFSEELEEDSOAFEAHL")
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let mut key: Vec<u8> = self.key.bytes().collect();
        let ptext = ptext.as_bytes();
        let mut matrix: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

        // populate matrix
        for i in 0..ptext.len() {
            matrix.entry(key[i % key.len()]).or_insert(vec![]);
            matrix.get_mut(&key[i % key.len()]).unwrap().push(ptext[i]);
        }

        key.sort();

        let mut ctext = vec![];
        for k in key.iter() {
            for byte in matrix.get(&k).unwrap() {
                ctext.push(*byte);
            }
        }

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::columnar_transposition::ColumnarTransposition;
    ///
    /// let key = String::from("GERMAN");
    /// let ct = ColumnarTransposition::new(key);
    ///
    /// let ptext = ct.decipher(String::from("NALCEHWTTDTTFSEELEEDSOAFEAHL"));
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let key: Vec<u8> = self.key.bytes().collect();
        let ctext = ctext.as_bytes();
        let mut matrix: HashMap<u8, Vec<u8>> = HashMap::with_capacity(key.len());

        let mut sorted_key = key.clone();
        sorted_key.sort();

        for k in sorted_key.iter() {
            matrix.insert(*k, vec![]);
        }

        // populate matrix
        let mut i = 0usize;
        let mut k = 0usize;
        while i < ctext.len() {
            // calculate length of current entry
            let col = key.iter().position(|&c| c == sorted_key[k]).unwrap();
            let len = if col < ctext.len() % key.len() {
                ctext.len() / key.len() + 1
            } else {
                ctext.len() / key.len()
            };

            for _ in 0..len {
                matrix.entry(sorted_key[k]).or_insert(vec![]);
                matrix.get_mut(&sorted_key[k]).unwrap().push(ctext[i]);
                i += 1;
            }

            k += 1;
        }

        let mut ptext = vec![];
        for i in 0..ctext.len() {
            ptext.push(matrix.get_mut(&key[i % key.len()]).unwrap().remove(0));
        }

        String::from_utf8(ptext).unwrap()
    }
}
