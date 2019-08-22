use ciphers::beaufort::Beaufort;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");
    let beaufort = Beaufort::new(key);

    let ctext = beaufort.encipher(ptext);
    assert_eq!(ctext, "CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("CKMPVCPVWPIWUJOGIUAPVWRIWUUK");
    let key = String::from("FORTIFICATION");
    let beaufort = Beaufort::new(key);

    let ptext = beaufort.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");
    let beaufort = Beaufort::new(key);

    let ctext = beaufort.encipher(ptext);
    assert_eq!(
        ctext,
        "ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("ZXVTRPNLJHFDBMKIGECAYWUSQOZXVTRPNLJHFDBMKIGECAYWUSQO");
    let key = String::from("ZYXWVUTSRQPON");
    let beaufort = Beaufort::new(key);

    let ptext = beaufort.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
