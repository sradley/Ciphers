use ciphers::rail_fence;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = rail_fence::cipher(plaintext, 4);
    assert_eq!(ciphertext, "DTTFSEDHSWOTATFNEAALHCLEELEE");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("DTTFSEDHSWOTATFNEAALHCLEELEE");

    let plaintext = rail_fence::decipher(ciphertext, 4);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ciphertext = rail_fence::cipher(plaintext, 6);
    assert_eq!(
        ciphertext,
        "AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT");

    let plaintext = rail_fence::decipher(ciphertext, 6);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
