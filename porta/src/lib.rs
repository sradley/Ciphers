//! # Porta Cipher
//! 
//! ...

static PORTA_TABLEU: [[char; 13]; 13] = [
    ['N','O','P','Q','R','S','T','U','V','W','X','Y','Z'],
    ['O','P','Q','R','S','T','U','V','W','X','Y','Z','N'],
    ['P','Q','R','S','T','U','V','W','X','Y','Z','N','O'],
    ['Q','R','S','T','U','V','W','X','Y','Z','N','O','P'],
    ['R','S','T','U','V','W','X','Y','Z','N','O','P','Q'],
    ['S','T','U','V','W','X','Y','Z','N','O','P','Q','R'],
    ['T','U','V','W','X','Y','Z','N','O','P','Q','R','S'],
    ['U','V','W','X','Y','Z','N','O','P','Q','R','S','T'],
    ['V','W','X','Y','Z','N','O','P','Q','R','S','T','U'],
    ['W','X','Y','Z','N','O','P','Q','R','S','T','U','V'],
    ['X','Y','Z','N','O','P','Q','R','S','T','U','V','W'],
    ['Y','Z','N','O','P','Q','R','S','T','U','V','W','X'],
    ['Z','N','O','P','Q','R','S','T','U','V','W','X','Y'],
];

#[cfg(test)]
mod tests {
    /// `cipher` test function ...
    #[test]
    fn cipher() {
        let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
        let key = String::from("FORTIFICATION");

        let ciphertext = crate::cipher(plaintext, key);
        assert_eq!(ciphertext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    }

    /// `decipher` test function ...
    #[test]
    fn decipher() {
        let ciphertext = String::from("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
        let key = String::from("FORTIFICATION");

        let plaintext = crate::decipher(ciphertext, key);
        assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
    }
}

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("FORTIFICATION");
///
/// let ciphertext = porta::cipher(plaintext, key);
/// assert_eq!(ciphertext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let key = key.as_bytes();

    let ciphertext = plaintext.bytes().enumerate()
        .map(move |(i, c)| {
            let y = (key[i % key.len()] as usize - 'A' as usize) / 2;

            if c - 'A' as u8 >= 13 {
                let mut x = 0u8;
                for j in 0..PORTA_TABLEU[y].len() {
                    if PORTA_TABLEU[y][j] as u8 == c {
                        x = j as u8;
                        break;
                    }
                }
                return x + 'A' as u8
            }

            PORTA_TABLEU[y][c as usize - 'A' as usize] as u8
        })
        .collect();

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
/// let key = String::from("FORTIFICATION");
///
/// let plaintext = porta::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    cipher(ciphertext, key)
}