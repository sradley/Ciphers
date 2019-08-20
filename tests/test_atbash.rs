use ciphers::atbash;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("ATTACKATDAWN");

    let ciphertext = atbash::cipher(plaintext);
    assert_eq!(ciphertext, "ZGGZXPZGWZDM");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("ZGGZXPZGWZDM");

    let plaintext = atbash::decipher(ciphertext);
    assert_eq!(plaintext, "ATTACKATDAWN");
}
