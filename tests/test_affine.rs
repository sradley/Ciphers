use ciphers::affine;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = affine::cipher(plaintext, 7, 11);
    assert_eq!(ciphertext, "GNUNYGOINNLHOJLKKFUOINZLHOKN");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("GNUNYGOINNLHOJLKKFUOINZLHOKN");

    let plaintext = affine::decipher(ciphertext, 7, 11);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}