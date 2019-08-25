use ciphers::{Cipher, CipherInputError, Porta};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let porta = Porta::new("FORTIFICATION");

    let ctext = porta.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let porta = Porta::new("FORTIFICATION");

    let ptext = porta.decipher("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let porta = Porta::new("ZYXWVUTSRQPON");

    let ctext = porta.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let porta = Porta::new("ZYXWVUTSRQPON");

    let ptext = porta.decipher("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let porta = Porta::new("fortification");

    let ctext = porta.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let porta = Porta::new("fortification");

    let ptext = porta.decipher("synnjscvrnrlahutukucvryrlany");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    Porta::new("fortific4ti0n");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let porta = Porta::new("fortification");

    let ctext = porta.encipher("defendtheeastw4llofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}
