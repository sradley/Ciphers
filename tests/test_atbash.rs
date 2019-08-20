use ciphers::atbash;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("ATTACKATDAWN");

    let ciphertext = atbash::cipher(plaintext);
    assert_eq!(ciphertext, "ZGGZXPZGWZDM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("ZGGZXPZGWZDM");

    let plaintext = atbash::decipher(ciphertext);
    assert_eq!(plaintext, "ATTACKATDAWN");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ciphertext = atbash::cipher(plaintext);
    assert_eq!(
        ciphertext,
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA");

    let plaintext = atbash::decipher(ciphertext);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
