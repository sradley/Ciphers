use ciphers::{Cipher, Substitution};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ctext = substitution.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ptext = substitution.decipher("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ctext = substitution.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let substitution = Substitution::new("PHQGIUMEAYLNOFDXJKRCVSTZWB");

    let ptext = substitution.decipher("PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ctext = substitution.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let substitution = Substitution::new("phqgiumeaylnofdxjkrcvstzwb");

    let ptext = substitution.decipher("giuifgceiiprctpnnduceiqprcni");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
