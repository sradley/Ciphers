use ciphers::caesar;

/// `cipher_rot_1` test function ...
#[test]
fn cipher_rot_1() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = caesar::cipher(plaintext, 1);
    assert_eq!(ciphertext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
}

/// `test_rot_25` test function ...
#[test]
fn cipher_rot_25() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = caesar::cipher(plaintext, 25);
    assert_eq!(ciphertext, "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
}

/// `decipher_rot_1` test function ...
#[test]
fn decipher_rot_1() {
    let ciphertext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");

    let plaintext = caesar::decipher(ciphertext, 1);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `decipher_rot_25` test function ...
#[test]
fn decipher_rot_25() {
    let ciphertext = String::from("CDEDMCSGDDZRSVZKKNESGDBZRSKD");

    let plaintext = caesar::decipher(ciphertext, 25);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}