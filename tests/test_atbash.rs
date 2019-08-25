use ciphers::{Atbash, Cipher, CipherInputError};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("ATTACKATDAWN");
    assert_eq!(ctext.unwrap(), "ZGGZXPZGWZDM");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("ZGGZXPZGWZDM");
    assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("attackatdawn");
    assert_eq!(ctext.unwrap(), "ZGGZXPZGWZDM");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("zggzxpzgwzdm");
    assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let atbash = Atbash::new();

    let ctext = atbash.encipher("4ttack4tdawn");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let atbash = Atbash::new();

    let ptext = atbash.decipher("z66zxpzgwzdm");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
