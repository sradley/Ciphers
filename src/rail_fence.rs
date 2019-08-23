//! # Rail-fence Cipher
//!
//! ...

use crate::Cipher;

/// `RailFence` struct ...
pub struct RailFence {
    key: usize,
}

impl RailFence {
    /// `RailFence` constructor ...
    pub fn new(key: usize) -> Self {
        Self { key }
    }
}

impl Cipher for RailFence {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::rail_fence::RailFence;
    ///
    /// let rail_fence = RailFence::new(4);
    ///
    /// let ctext = rail_fence.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    /// assert_eq!(ctext, "DTTFSEDHSWOTATFNEAALHCLEELEE");
    /// ```
    fn encipher(&self, ptext: &str) -> String {
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

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::rail_fence::RailFence;
    ///
    /// let rail_fence = RailFence::new(4);
    ///
    /// let ptext = rail_fence.decipher("DTTFSEDHSWOTATFNEAALHCLEELEE");
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: &str) -> String {
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

        String::from_utf8(ptext).unwrap()
    }
}
