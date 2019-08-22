use ciphers::columnar_transposition::ColumnarTransposition;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("GERMAN");
    let ct = ColumnarTransposition::new(key);

    let ctext = ct.encipher(ptext);
    assert_eq!(ctext, "NALCEHWTTDTTFSEELEEDSOAFEAHL")
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("NALCEHWTTDTTFSEELEEDSOAFEAHL");
    let key = String::from("GERMAN");
    let ct = ColumnarTransposition::new(key);

    let ptext = ct.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");
    let ct = ColumnarTransposition::new(key);

    let ctext = ct.encipher(ptext);
    assert_eq!(
        ctext,
        "MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN");
    let key = String::from("ZYXWVUTSRQPON");
    let ct = ColumnarTransposition::new(key);

    let ptext = ct.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
