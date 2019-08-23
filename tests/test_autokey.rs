use ciphers::{Autokey, Cipher};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let autokey = Autokey::new("FORTIFICATION");

    let ctext = autokey.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let autokey = Autokey::new("FORTIFICATION");

    let ptext = autokey.decipher("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let autokey = Autokey::new("ZYXWVUTSRQPON");

    let ctext = autokey.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let autokey = Autokey::new("ZYXWVUTSRQPON");

    let ptext = autokey.decipher("ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
