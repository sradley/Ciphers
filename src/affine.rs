//! # Affine Cipher
//!
//! ...

use crate::Cipher;

/// `Affine` struct ...
pub struct Affine {
    a: i32,
    b: i32,
}

impl Affine {
    /// `Affine` constructor ...
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }
}

impl Cipher for Affine {
    /// `encipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::affine::Affine;
    ///
    /// let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    /// let affine = Affine::new(7, 11);
    ///
    /// let ctext = affine.encipher(ptext);
    /// assert_eq!(ctext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
    /// ```
    fn encipher(&self, ptext: String) -> String {
        let ctext = ptext
            .bytes()
            .map(move |c| ((self.a * (c as i32 - 65) + self.b) % 26) as u8 + 65)
            .collect();

        String::from_utf8(ctext).unwrap()
    }

    /// `decipher` method ...
    ///
    /// ```
    /// use ciphers::Cipher;
    /// use ciphers::affine::Affine;
    ///
    /// let ctext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");
    /// let affine = Affine::new(7, 11);
    ///
    /// let ptext = affine.decipher(ctext);
    /// assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
    /// ```
    fn decipher(&self, ctext: String) -> String {
        let a_inv = invmod(self.a, 26).unwrap();

        let ptext = ctext
            .bytes()
            .map(move |c| (((a_inv * (c as i32 - 65 - self.b)) % 26 + 26) % 26) as u8 + 65)
            .collect();

        String::from_utf8(ptext).unwrap()
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
