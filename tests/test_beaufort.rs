use ciphers::{Beaufort, Cipher};

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
