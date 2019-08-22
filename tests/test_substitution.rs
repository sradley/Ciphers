use ciphers::substitution::Substitution;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    let substitution = Substitution::new(key);

    let ctext = substitution.encipher(ptext);
    assert_eq!(ctext, "GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("GIUIFGCEIIPRCTPNNDUCEIQPRCNI");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    let substitution = Substitution::new(key);

    let ptext = substitution.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    let substitution = Substitution::new(key);

    let ctext = substitution.encipher(ptext);
    assert_eq!(
        ctext,
        "PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWBPHQGIUMEAYLNOFDXJKRCVSTZWB");
    let key = String::from("PHQGIUMEAYLNOFDXJKRCVSTZWB");
    let substitution = Substitution::new(key);

    let ptext = substitution.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
