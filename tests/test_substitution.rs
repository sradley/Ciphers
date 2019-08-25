use ciphers::{Cipher, CipherInputError, Substitution};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ctext = substitution.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ptext = substitution.decipher("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ctext = substitution.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ptext = substitution.decipher("PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ctext = substitution.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ptext = substitution.decipher("giuifgceiiprctpnnduceiqprcni");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_not_26_chars` test function.
#[test]
#[should_panic]
fn key_not_26_chars() {
    Substitution::new("phqgiumeaylnofdxjkrcvstzw");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    Substitution::new("phqg1umeaylnofdxjkrcvstzwb");
}

/// `key_repeated_chars` test function.
#[test]
#[should_panic]
fn key_repeated_chars() {
    Substitution::new("phqgiumeaylnofdxikrcvstzwb");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ctext = substitution.encipher("defendth33astwallofthec4stle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ptext = substitution.decipher("giuifgce11prctpnnduceiqprcni");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
