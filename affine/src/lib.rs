//! # Affine Cipher
//!
//! ...

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

        let ciphertext = crate::cipher(plaintext, 7, 11);
        assert_eq!(ciphertext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");

        let plaintext = crate::decipher(ciphertext, 7, 11);
        assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
///
/// let ciphertext = affine::cipher(plaintext, 7, 11);
/// assert_eq!(ciphertext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
/// ```
pub fn cipher(plaintext: String, a: u16, b: u16) -> String {
    let plaintext = plaintext
        .bytes()
        .map(move |c| ((a * (c as u16 - 'A' as u16) + b) % 26) as u8 + 'A' as u8)
        .collect();

    String::from_utf8(plaintext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");
///
/// let plaintext = affine::decipher(ciphertext, 7, 11);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, a: i32, b: i32) -> String {
    let a_inv = invmod(a, 26).unwrap();

    let ciphertext = ciphertext
        .bytes()
        .map(move |c| (((a_inv * (c as i32 - 'A' as i32 - b)) % 26 + 26) % 26) as u8 + 'A' as u8)
        .collect();

    String::from_utf8(ciphertext).unwrap()
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
