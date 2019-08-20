use ciphers::autokey;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = autokey::cipher(plaintext, key);
    assert_eq!(ciphertext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    let key = String::from("FORTIFICATION");

    let plaintext = autokey::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");

    let ciphertext = autokey::cipher(plaintext, key);
    assert_eq!(ciphertext, "ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL");
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL");
    let key = String::from("ZYXWVUTSRQPON");

    let plaintext = autokey::decipher(ciphertext, key);
    assert_eq!(plaintext, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
}