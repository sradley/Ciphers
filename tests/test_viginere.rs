use ciphers::vigenere::Vigenere;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");
    let vigenere = Vigenere::new(key);

    let ciphertext = vigenere.encipher(plaintext);
    assert_eq!(ciphertext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    let key = String::from("FORTIFICATION");
    let vigenere = Vigenere::new(key);

    let plaintext = vigenere.decipher(ciphertext);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("AYLNOFDXJKRCVSTZWB");
    let vigenere = Vigenere::new(key);

    let ciphertext = vigenere.encipher(plaintext);
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
    let vigenere = Vigenere::new(key);

    let plaintext = vigenere.decipher(ciphertext);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
