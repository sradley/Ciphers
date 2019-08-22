use ciphers::atbash::Atbash;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("ATTACKATDAWN");
    let atbash = Atbash::new();

    let ctext = atbash.encipher(ptext);
    assert_eq!(ctext, "ZGGZXPZGWZDM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("ZGGZXPZGWZDM");
    let atbash = Atbash::new();

    let ptext = atbash.decipher(ctext);
    assert_eq!(ptext, "ATTACKATDAWN");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let atbash = Atbash::new();

    let ctext = atbash.encipher(ptext);
    assert_eq!(
        ctext,
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA");
    let atbash = Atbash::new();

    let ptext = atbash.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
