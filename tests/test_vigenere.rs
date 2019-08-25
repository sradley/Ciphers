use ciphers::{Cipher, CipherInputError, Vigenere};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let vigenere = Vigenere::new("FORTIFICATION");

    let ctext = vigenere.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let vigenere = Vigenere::new("FORTIFICATION");

    let ptext = vigenere.decipher("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let vigenere = Vigenere::new("AYLNOFDXJKRCVSTZWB");

    let ctext = vigenere.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let vigenere = Vigenere::new("AYLNOFDXJKRCVSTZWB");

    let ptext = vigenere.decipher("AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let vigenere = Vigenere::new("fortification");

    let ctext = vigenere.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let vigenere = Vigenere::new("fortification");

    let ptext = vigenere.decipher("iswxvibjexiggbocewkbjeviggqs");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    Vigenere::new("fortific4tion");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let vigenere = Vigenere::new("fortification");

    let ctext = vigenere.encipher("def3ndtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let vigenere = Vigenere::new("fortification");

    let ptext = vigenere.decipher("iswxvibjexiggbocewkbj3viggqs");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
