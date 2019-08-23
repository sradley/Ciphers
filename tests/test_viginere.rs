use ciphers::vigenere::Vigenere;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");
    let vigenere = Vigenere::new(key);

    let ctext = vigenere.encipher(ptext);
    assert_eq!(ctext, "ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("ISWXVIBJEXIGGBOCEWKBJEVIGGQS");
    let key = String::from("FORTIFICATION");
    let vigenere = Vigenere::new(key);

    let ptext = vigenere.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("AYLNOFDXJKRCVSTZWB");
    let vigenere = Vigenere::new(key);

    let ctext = vigenere.encipher(ptext);
    assert_eq!(
        ctext,
        "AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("AZNQSKJERTBNHFHOMSSRFIKCBWJLTFZXZGEKKJXACUTOBDLXRPRY");
    let key = String::from("AYLNOFDXJKRCVSTZWB");
    let vigenere = Vigenere::new(key);

    let ptext = vigenere.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
