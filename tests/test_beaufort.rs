use ciphers::{Beaufort, Cipher, CipherInputError};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let beaufort = Beaufort::new("FORTIFICATION");

    let ctext = beaufort.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let beaufort = Beaufort::new("FORTIFICATION");

    let ptext = beaufort.decipher("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let beaufort = Beaufort::new("ZYXWVUTSRQPON");

    let ctext = beaufort.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let beaufort = Beaufort::new("ZYXWVUTSRQPON");

    let ptext = beaufort.decipher("ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let beaufort = Beaufort::new("fortification");

    let ctext = beaufort.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let beaufort = Beaufort::new("fortification");

    let ptext = beaufort.decipher("ckmpvcpvwpiwujogiuapvwriwuuk");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    Beaufort::new("f0rtification");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let beaufort = Beaufort::new("fortification");

    let ctext = beaufort.encipher("d3f3ndtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}
