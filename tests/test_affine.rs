use ciphers::affine::Affine;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher(ptext);
    assert_eq!(ctext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let affine = Affine::new(3, 13);

    let ctext = affine.encipher(ptext);
    assert_eq!(
        ctext,
        "NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK");
    let affine = Affine::new(3, 13);

    let ptext = affine.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
