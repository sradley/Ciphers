use ciphers::porta;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = porta::cipher(plaintext, key);
    assert_eq!(ciphertext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    let key = String::from("FORTIFICATION");

    let plaintext = porta::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}