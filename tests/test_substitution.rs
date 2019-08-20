use ciphers::substitution;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ciphertext = substitution::cipher(plaintext, key);
    assert_eq!(ciphertext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let plaintext = substitution::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
