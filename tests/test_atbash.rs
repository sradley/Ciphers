use ciphers::{Atbash, Cipher};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("ATTACKATDAWN");
    assert_eq!(ctext, "ZGGZXPZGWZDM");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("ZGGZXPZGWZDM");
    assert_eq!(ptext, "ATTACKATDAWN");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("attackatdawn");
    assert_eq!(ctext, "ZGGZXPZGWZDM");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("zggzxpzgwzdm");
    assert_eq!(ptext, "ATTACKATDAWN");
}
