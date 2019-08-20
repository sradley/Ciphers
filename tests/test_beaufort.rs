use ciphers::beaufort;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = beaufort::cipher(plaintext, key);
    assert_eq!(ciphertext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    let key = String::from("FORTIFICATION");

    let plaintext = beaufort::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}