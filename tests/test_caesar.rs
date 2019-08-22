use ciphers::Cipher;
use ciphers::caesar::Caesar;

/// `encipher_rot1_small` test function ...
#[test]
fn encipher_rot1_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher(ptext);
    assert_eq!(ctext, "EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
}

/// `decipher_rot1_small` test function ...
#[test]
fn decipher_rot1_small() {
    let ctext = String::from("EFGFOEUIFFBTUXBMMPGUIFDBTUMF");
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_rot25_small` test function ...
#[test]
fn encipher_rot25_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let caesar = Caesar::new(25);

    let ctext = caesar.encipher(ptext);
    assert_eq!(ctext, "CDEDMCSGDDZRSVZKKNESGDBZRSKD");
}

/// `decipher_rot25_small` test function ...
#[test]
fn decipher_rot25_small() {
    let ctext = String::from("CDEDMCSGDDZRSVZKKNESGDBZRSKD");
    let caesar = Caesar::new(25);

    let ptext = caesar.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_rot1_large` test function ...
#[test]
fn encipher_rot1_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher(ptext);
    assert_eq!(ctext, "BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA");
}

/// `decipher_rot1_large` test function ...
#[test]
fn decipher_rot1_large() {
    let ctext = String::from("BCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZA");
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher(ctext);
    assert_eq!(ptext, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

/// `encipher_rot25_large` test function ...
#[test]
fn encipher_rot25_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let caesar = Caesar::new(25);

    let ctext = caesar.encipher(ptext);
    assert_eq!(ctext, "ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY");
}

/// `decipher_rot25_large` test function ...
#[test]
fn decipher_rot25_large() {
    let ctext = String::from("ZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXY");
    let caesar = Caesar::new(25);

    let ptext = caesar.decipher(ctext);
    assert_eq!(ptext, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
}
