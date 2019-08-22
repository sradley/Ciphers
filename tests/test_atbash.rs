use ciphers::Cipher;
use ciphers::atbash::Atbash;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let plaintext = String::from("ATTACKATDAWN");
    let atbash = Atbash::new();

    let ciphertext = atbash.encipher(plaintext);
    assert_eq!(ciphertext, "ZGGZXPZGWZDM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("ZGGZXPZGWZDM");
    let atbash = Atbash::new();

    let plaintext = atbash.decipher(ciphertext);
    assert_eq!(plaintext, "ATTACKATDAWN");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let atbash = Atbash::new();

    let ciphertext = atbash.encipher(plaintext);
    assert_eq!(
        ciphertext,
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA");
    let atbash = Atbash::new();

    let plaintext = atbash.decipher(ciphertext);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
