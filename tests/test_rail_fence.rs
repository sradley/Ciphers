use ciphers::rail_fence;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = rail_fence::cipher(plaintext, 4);
    assert_eq!(ciphertext, "DTTFSEDHSWOTATFNEAALHCLEELEE");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("DTTFSEDHSWOTATFNEAALHCLEELEE");

    let plaintext = rail_fence::decipher(ciphertext, 4);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}