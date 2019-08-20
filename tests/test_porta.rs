use ciphers::porta;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = porta::cipher(plaintext, key);
    assert_eq!(ciphertext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    let key = String::from("FORTIFICATION");

    let plaintext = porta::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");

    let ciphertext = porta::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG");
    let key = String::from("ZYXWVUTSRQPON");

    let plaintext = porta::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
