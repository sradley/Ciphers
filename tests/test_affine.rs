use ciphers::affine;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = affine::cipher(plaintext, 7, 11);
    assert_eq!(ciphertext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");

    let plaintext = affine::decipher(ciphertext, 7, 11);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ciphertext = affine::cipher(plaintext, 3, 13);
    assert_eq!(
        ciphertext,
        "NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("NQTWZCFILORUXADGJMPSVYBEHKNQTWZCFILORUXADGJMPSVYBEHK");

    let plaintext = affine::decipher(ciphertext, 3, 13);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
