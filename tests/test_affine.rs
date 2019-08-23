use ciphers::{Affine, Cipher};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher("GNUNYGOINNLHOJLKKFUOINZLHOKN");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let affine = Affine::new(3, 13);

    let ctext = affine.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let affine = Affine::new(3, 13);

    let ptext = affine.decipher("NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher("gnunygoinnlhojlkkfuoinzlhokn");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}