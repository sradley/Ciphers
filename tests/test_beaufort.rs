use ciphers::beaufort;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = beaufort::cipher(plaintext, key);
    assert_eq!(ciphertext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    let key = String::from("FORTIFICATION");

    let plaintext = beaufort::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");

    let ciphertext = beaufort::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO");
    let key = String::from("ZYXWVUTSRQPON");

    let plaintext = beaufort::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
