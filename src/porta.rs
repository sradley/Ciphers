//! # Porta Cipher
//!
//! ...

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

/// `cipher` function ...
///
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("FORTIFICATION");
///
/// let ciphertext = ciphers::porta::cipher(plaintext, key);
/// assert_eq!(ciphertext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    let key = key.as_bytes();

    let ciphertext = plaintext
        .bytes()
        .enumerate()
        .map(move |(i, c)| {
            let y = (key[i % key.len()] as usize - 'A' as usize) / 2;
            match c {
                78...90 => PORTA_TABLEU[y].iter().position(|&j| j == c).unwrap() as u8 + 'A' as u8,
                _ => PORTA_TABLEU[y][c as usize - 'A' as usize] as u8,
            }
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
/// let plaintext = ciphers::porta::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    cipher(ciphertext, key)
}