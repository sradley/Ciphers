use ciphers::{Affine, Cipher, CipherInputError};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher("GNUNYGOINNLHOJLKKFUOINZLHOKN");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let affine = Affine::new(3, 13);

    let ctext = affine.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let affine = Affine::new(3, 13);

    let ptext = affine.decipher("NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher("gnunygoinnlhojlkkfuoinzlhokn");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `a_leq_0` test function.
#[test]
#[should_panic]
fn a_leq_0() {
    Affine::new(0, 11);
}

/// `a_geq_26` test functin.
#[test]
#[should_panic]
fn a_geq_26() {
    Affine::new(26, 11);
}

/// `b_less_0` test function.
#[test]
#[should_panic]
fn b_less_0() {
    Affine::new(7, -1);
}

/// `b_geq_26` test function.
#[test]
#[should_panic]
fn b_geq_26() {
    Affine::new(7, 26);
}

/// `a_not_relatively_prime` test function.
#[test]
#[should_panic]
fn a_not_relatively_prime() {
    Affine::new(2, 11);
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let affine = Affine::new(7, 11);

    let ctext = affine.encipher("d3fendtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let affine = Affine::new(7, 11);

    let ptext = affine.decipher("6nunygo1nnlhojlkkfuoinzlhokn");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
