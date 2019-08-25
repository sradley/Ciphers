use ciphers::{Autokey, Cipher, CipherInputError};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let autokey = Autokey::new("FORTIFICATION");

    let ctext = autokey.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let autokey = Autokey::new("FORTIFICATION");

    let ptext = autokey.decipher("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let autokey = Autokey::new("ZYXWVUTSRQPON");

    let ctext = autokey.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let autokey = Autokey::new("ZYXWVUTSRQPON");

    let ptext = autokey.decipher("ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let autokey = Autokey::new("fortification");

    let ctext = autokey.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let autokey = Autokey::new("fortification");

    let ptext = autokey.decipher("iswxvibjexiggzeqpbimoigakmhe");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    Autokey::new("f0rtificati0n");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let autokey = Autokey::new("fortification");

    let ctext = autokey.encipher("d3f3ndtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let autokey = Autokey::new("fortification");

    let ptext = autokey.decipher("15wxvibjexiggzeqpbimoigakmh3");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
