use ciphers::vigenere;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = vigenere::cipher(plaintext, key);
    assert_eq!(ciphertext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    let key = String::from("FORTIFICATION");

    let plaintext = vigenere::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}