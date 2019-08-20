use ciphers::substitution;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ciphertext = substitution::cipher(plaintext, key);
    assert_eq!(ciphertext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let plaintext = substitution::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ciphertext = substitution::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let plaintext = substitution::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
