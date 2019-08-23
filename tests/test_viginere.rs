use ciphers::{Cipher, Vigenere};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let vigenere = Vigenere::new("FORTIFICATION");

    let ctext = vigenere.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let vigenere = Vigenere::new("FORTIFICATION");

    let ptext = vigenere.decipher("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let vigenere = Vigenere::new("AYLNOFDXJKRCVSTZWB");

    let ctext = vigenere.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let vigenere = Vigenere::new("AYLNOFDXJKRCVSTZWB");

    let ptext = vigenere.decipher("AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let vigenere = Vigenere::new("fortification");

    let ctext = vigenere.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let vigenere = Vigenere::new("fortification");

    let ptext = vigenere.decipher("iswxvibjexiggbocewkbjeviggqs");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
