use ciphers::{Caesar, Cipher};

/// `encipher_rot1_small` test function.
#[test]
fn encipher_rot1_small() {
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
}

/// `decipher_rot1_small` test function.
#[test]
fn decipher_rot1_small() {
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_rot25_small` test function.
#[test]
fn encipher_rot25_small() {
    let caesar = Caesar::new(25);

    let ctext = caesar.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
}

/// `decipher_rot25_small` test function.
#[test]
fn decipher_rot25_small() {
    let caesar = Caesar::new(25);

    let ptext = caesar.decipher("CDEDMCSGDDZRSVZKKNESGDBZRSKD");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_rot1_large` test function.
#[test]
fn encipher_rot1_large() {
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA"
    );
}

/// `decipher_rot1_large` test function.
#[test]
fn decipher_rot1_large() {
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher("BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_rot25_large` test function.
#[test]
fn encipher_rot25_large() {
    let caesar = Caesar::new(25);

    let ctext = caesar.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY"
    );
}

/// `decipher_rot25_large` test function.
#[test]
fn decipher_rot25_large() {
    let caesar = Caesar::new(25);

    let ptext = caesar.decipher("ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher("efgfoeuiffbtuxbmmpguifdbtumf");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}
