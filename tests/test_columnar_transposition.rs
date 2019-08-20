use ciphers::columnar_transposition;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("GERMAN");

    let ciphertext = columnar_transposition::cipher(plaintext, key);
    assert_eq!(ciphertext, "NALCXEHWTTDTTFSEELEEDSOAXFEAHL")
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("NALCXEHWTTDTTFSEELEEDSOAXFEAHL");
    let key = String::from("GERMAN");

    let plaintext = columnar_transposition::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLEXX");
}
