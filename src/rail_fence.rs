//! # Rail-fence Cipher
//!
//! ...

/// `cipher` function ...
/// 
/// ```
/// let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
///
/// let ciphertext = ciphers::rail_fence::cipher(plaintext, 4);
/// assert_eq!(ciphertext, "DTTFSEDHSWOTATFNEAALHCLEELEE");
/// ```
pub fn cipher(plaintext: String, key: usize) -> String {
    let mut ciphertext = vec![];
    let plaintext: Vec<u8> = plaintext.bytes().collect();

    let mut line = 0usize;
    while line < key - 1 {
        let skip = 2 * (key - line - 1);
        let mut j = 0usize;

        let mut i = line;
        while i < plaintext.len() {
            ciphertext.push(*plaintext.get(i).unwrap());

            if line == 0 || j % 2 == 0 {
                i += skip;
            } else {
                i += 2 * (key - 1) - skip;
            }

            j += 1;
        }

        line += 1;
    }

    for i in (line..plaintext.len()).step_by(2 * (key - 1)) {
        ciphertext.push(*plaintext.get(i).unwrap());
    }

    String::from_utf8(ciphertext).unwrap()
}

/// `decipher` function ...
/// 
/// ```
/// let ciphertext = String::from("DTTFSEDHSWOTATFNEAALHCLEELEE");
/// 
/// let plaintext = ciphers::rail_fence::decipher(ciphertext, 4);
/// assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
/// ```
pub fn decipher(ciphertext: String, key: usize) -> String {
    let mut plaintext = vec![0u8; ciphertext.len()];
    let ciphertext: Vec<u8> = ciphertext.bytes().collect();
    let mut k = 0usize;

    let mut line = 0usize;
    while line < key - 1 {
        let skip = 2 * (key - line - 1);
        let mut j = 0usize;

        let mut i = line;
        while i < ciphertext.len() {
            plaintext[i] = *ciphertext.get(k).unwrap();
            k += 1;

            if line == 0 || j % 2 == 0 {
                i += skip;
            } else {
                i += 2 * (key - 1) - skip;
            }

            j += 1;
        }

        line += 1;
    }

    for i in (line..ciphertext.len()).step_by(2 * (key - 1)) {
        plaintext[i] = *ciphertext.get(k).unwrap();
        k += 1;
    }

    String::from_utf8(plaintext).unwrap()
}
