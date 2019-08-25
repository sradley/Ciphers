use ciphers::{Caesar, Cipher, CipherInputError};

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

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let caesar = Caesar::new(1);

    let ctext = caesar.encipher("d3f3ndtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let caesar = Caesar::new(1);

    let ptext = caesar.decipher("3f6fo3uiffbtuxbmmpguifdbtumf");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}
