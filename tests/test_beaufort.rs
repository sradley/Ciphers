use ciphers::beaufort::Beaufort;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let beaufort = Beaufort::new("FORTIFICATION");

    let ctext = beaufort.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let beaufort = Beaufort::new("FORTIFICATION");

    let ptext = beaufort.decipher("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let beaufort = Beaufort::new("ZYXWVUTSRQPON");

    let ctext = beaufort.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let beaufort = Beaufort::new("ZYXWVUTSRQPON");

    let ptext = beaufort.decipher("ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
