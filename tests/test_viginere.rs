use ciphers::vigenere;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");

    let ciphertext = vigenere::cipher(plaintext, key);
    assert_eq!(ciphertext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    let key = String::from("FORTIFICATION");

    let plaintext = vigenere::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("AYLNOFDXJKRCVSTZWB");

    let ciphertext = vigenere::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY");
    let key = String::from("AYLNOFDXJKRCVSTZWB");

    let plaintext = vigenere::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
