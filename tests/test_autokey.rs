use ciphers::autokey::Autokey;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");
    let autokey = Autokey::new(key);

    let ctext = autokey.encipher(ptext);
    assert_eq!(ctext, "ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("ISWXVIBJEXIGGZEQPBIMOIGAKMHE");
    let key = String::from("FORTIFICATION");
    let autokey = Autokey::new(key);

    let ptext = autokey.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");
    let autokey = Autokey::new(key);

    let ctext = autokey.encipher(ptext);
    assert_eq!(
        ctext,
        "ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("ZZZZZZZZZZZZZNPRTVXZBDFHJLNPRTVXZBDFHJLNPRTVXZBDFHJL");
    let key = String::from("ZYXWVUTSRQPON");
    let autokey = Autokey::new(key);

    let ptext = autokey.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
