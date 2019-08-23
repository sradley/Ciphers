use ciphers::columnar_transposition::ColumnarTransposition;
use ciphers::Cipher;

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let ct = ColumnarTransposition::new("GERMAN");

    let ctext = ct.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "NALCEHWTTDTTFSEELEEDSOAFEAHL")
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let ct = ColumnarTransposition::new("GERMAN");

    let ptext = ct.decipher("NALCEHWTTDTTFSEELEEDSOAFEAHL");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let ct = ColumnarTransposition::new("ZYXWVUTSRQPON");

    let ctext = ct.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let ct = ColumnarTransposition::new("ZYXWVUTSRQPON");

    let ptext = ct.decipher("MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
