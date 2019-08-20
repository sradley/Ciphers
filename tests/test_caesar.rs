use ciphers::caesar;

/// `cipher_rot1_small` test function ...
#[test]
fn cipher_rot1_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = caesar::cipher(plaintext, 1);
    assert_eq!(ciphertext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
}

/// `decipher_rot1_small` test function ...
#[test]
fn decipher_rot1_small() {
    let ciphertext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");

    let plaintext = caesar::decipher(ciphertext, 1);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `test_rot25_small` test function ...
#[test]
fn cipher_rot25_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");

    let ciphertext = caesar::cipher(plaintext, 25);
    assert_eq!(ciphertext, "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
}

/// `decipher_rot25_small` test function ...
#[test]
fn decipher_rot25_small() {
    let ciphertext = String::from("CDEDMCSGDDZRSVZKKNESGDBZRSKD");

    let plaintext = caesar::decipher(ciphertext, 25);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_rot1_large` test function ...
#[test]
fn cipher_rot1_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ciphertext = caesar::cipher(plaintext, 1);
    assert_eq!(ciphertext, "BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA");
}

/// `decipher_rot1_large` test function ...
#[test]
fn decipher_rot1_large() {
    let ciphertext = String::from("BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA");

    let plaintext = caesar::decipher(ciphertext, 1);
    assert_eq!(plaintext, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

/// `test_rot25_large` test function ...
#[test]
fn cipher_rot25_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ciphertext = caesar::cipher(plaintext, 25);
    assert_eq!(ciphertext, "ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY");
}

/// `decipher_rot25_large` test function ...
#[test]
fn decipher_rot25_large() {
    let ciphertext = String::from("ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY");

    let plaintext = caesar::decipher(ciphertext, 25);
    assert_eq!(plaintext, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
}