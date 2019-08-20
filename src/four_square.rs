//! # Four-Square Cipher
//!
//! ...
//! 
//! TODO: handle unwraps (i.e. when trying to find a letter that's not in the alphabet)

static ALPHABET: [u8; 25] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90,
];

/// `cipher` function ...
///
/// ```
/// use ciphers::four_square;
///
/// let plaintext = String::from("ATTACKATDAWN");
/// let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
/// let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
///
/// let ciphertext = four_square::cipher(plaintext, key1, key2);
/// assert_eq!(ciphertext, "TIYBFHTIZBSY");
/// ```
pub fn cipher(plaintext: String, key1: String, key2: String) -> String {
    assert_eq!(key1.len(), 25);
    assert_eq!(key2.len(), 25);

    let mut plaintext: Vec<u8> = plaintext.bytes().collect();
    if plaintext.len() % 2 != 0 {
        plaintext.push('X' as u8);
    }

    let key1 = key1.as_bytes();
    let key2 = key2.as_bytes();

    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for i in (0..plaintext.len()).step_by(2) {
        let yx1 = ALPHABET.iter().position(|&c| c == plaintext[i]).unwrap();
        let yx2 = ALPHABET
            .iter()
            .position(|&c| c == plaintext[i + 1])
            .unwrap();

        let (y1, x1) = (yx1 / 5, yx1 % 5);
        let (y2, x2) = (yx2 / 5, yx2 % 5);

        ciphertext.push(key1[y1 * 5 + x2]);
        ciphertext.push(key2[y2 * 5 + x1]);
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// use ciphers::four_square;
///
/// let ciphertext = String::from("TIYBFHTIZBSY");
/// let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
/// let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
///
/// let plaintext = four_square::decipher(ciphertext, key1, key2);
/// assert_eq!(plaintext, "ATTACKATDAWN");
/// ```
pub fn decipher(ciphertext: String, key1: String, key2: String) -> String {
    assert_eq!(key1.len(), 25);
    assert_eq!(key2.len(), 25);
    assert_eq!(ciphertext.len() % 2, 0);

    let ciphertext = ciphertext.as_bytes();

    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for i in (0..ciphertext.len()).step_by(2) {
        let yx1 = key1.find(|c| c == ciphertext[i] as char).unwrap();
        let yx2 = key2.find(|c| c == ciphertext[i + 1] as char).unwrap();

        let (y1, x2) = (yx1 / 5, yx1 % 5);
        let (y2, x1) = (yx2 / 5, yx2 % 5);

        plaintext.push(ALPHABET[y1 * 5 + x1]);
        plaintext.push(ALPHABET[y2 * 5 + x2]);
    }

    String::from_utf8(plaintext).unwrap()
}
