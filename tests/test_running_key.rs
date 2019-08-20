use ciphers::running_key;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ciphertext = running_key::cipher(plaintext, key);
    assert_eq!(ciphertext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let plaintext = running_key::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
