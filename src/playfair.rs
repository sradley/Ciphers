//! # Playfair Cipher
//!
//! ...

/// `cipher` function ...
///
/// ```
/// use ciphers::playfair;
///
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
/// let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
///
/// let ciphertext = playfair::cipher(plaintext, key);
/// assert_eq!(ciphertext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
/// ```
pub fn cipher(plaintext: String, key: String) -> String {
    assert_eq!(key.len(), 25);

    let mut plaintext: Vec<u8> = plaintext.bytes().collect();
    if plaintext.len() % 2 != 0 {
        plaintext.push('X' as u8);
    }

    let key = key.as_bytes();

    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for i in (0..plaintext.len()).step_by(2) {
        if plaintext[i] == plaintext[i + 1] {
            plaintext[i + 1] = 'X' as u8;
        }

        let yx1 = key.iter().position(|&c| c == plaintext[i]).unwrap();
        let yx2 = key.iter().position(|&c| c == plaintext[i + 1]).unwrap();

        let (y1, x1) = (yx1 / 5, yx1 % 5);
        let (y2, x2) = (yx2 / 5, yx2 % 5);

        if y1 != y2 && x1 != x2 {
            // different rows and columns
            ciphertext.push(key[y1 * 5 + x2]);
            ciphertext.push(key[y2 * 5 + x1]);
        } else if y1 == y2 {
            // same row
            ciphertext.push(key[y1 * 5 + (x1 + 1) % 5]);
            ciphertext.push(key[y2 * 5 + (x2 + 1) % 5]);
        } else if x1 == x2 {
            // same column
            ciphertext.push(key[(y1 + 1) % 5 * 5 + x1]);
            ciphertext.push(key[(y2 + 1) % 5 * 5 + x2]);
        }
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
///
/// ```
/// use ciphers::playfair;
///
/// let ciphertext = String::from("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
/// let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
///
/// let plaintext = playfair::decipher(ciphertext, key);
/// assert_eq!(plaintext, "DEFENDTHEXASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: String) -> String {
    assert_eq!(key.len(), 25);

    let ciphertext = ciphertext.as_bytes();
    let key = key.as_bytes();

    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for i in (0..ciphertext.len()).step_by(2) {
        let yx1 = key.iter().position(|&c| c == ciphertext[i]).unwrap();
        let yx2 = key.iter().position(|&c| c == ciphertext[i + 1]).unwrap();

        let (y1, x1) = (yx1 / 5, yx1 % 5);
        let (y2, x2) = (yx2 / 5, yx2 % 5);

        if y1 != y2 && x1 != x2 {
            // different rows and columns
            plaintext.push(key[y1 * 5 + x2]);
            plaintext.push(key[y2 * 5 + x1]);
        } else if y1 == y2 {
            // same row
            plaintext.push(key[y1 * 5 + (x1 + 5 - 1) % 5]);
            plaintext.push(key[y2 * 5 + (x2 + 5 - 1) % 5]);
        } else if x1 == x2 {
            // same column
            plaintext.push(key[(y1 + 5 - 1) % 5 * 5 + x1]);
            plaintext.push(key[(y2 + 5 - 1) % 5 * 5 + x2]);
        }
    }

    String::from_utf8(plaintext).unwrap()
}
