use ciphers::{Cipher, CipherInputError, ColumnarTransposition};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let ct = ColumnarTransposition::new("GERMAN");

    let ctext = ct.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "NALCEHWTTDTTFSEELEEDSOAFEAHL");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let ct = ColumnarTransposition::new("GERMAN");

    let ptext = ct.decipher("NALCEHWTTDTTFSEELEEDSOAFEAHL");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let ct = ColumnarTransposition::new("ZYXWVUTSRQPON");

    let ctext = ct.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let ct = ColumnarTransposition::new("ZYXWVUTSRQPON");

    let ptext = ct.decipher("MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let ct = ColumnarTransposition::new("german");

    let ctext = ct.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "nalcehwttdttfseeleedsoafeahl");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let ct = ColumnarTransposition::new("german");

    let ptext = ct.decipher("nalcehwttdttfseeleedsoafeahl");
    assert_eq!(ptext.unwrap(), "defendtheeastwallofthecastle");
}

/// `key_non_ascii` test function.
#[test]
#[should_panic]
fn key_non_ascii() {
    ColumnarTransposition::new("gèrman");
}

/// `ptext_non_ascii` test function.
#[test]
fn ptext_non_ascii() {
    let ct = ColumnarTransposition::new("german");

    let ctext = ct.encipher("dèfèndthèèastwallofthècastlè");
    assert_eq!(ctext, Err(CipherInputError::NotAscii));
}

/// `ctext_non_ascii` test function.
#[test]
fn ctext_non_ascii() {
    let ct = ColumnarTransposition::new("german");

    let ptext = ct.decipher("nalcèhwttdttfsèèlèèdsoafèahl");
    assert_eq!(ptext, Err(CipherInputError::NotAscii));
}
