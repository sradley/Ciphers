use ciphers::autokey;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = autokey::cipher(plaintext, key);
    assert_eq!(ciphertext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    let key = String::from("FORTIFICATION");

    let plaintext = autokey::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}