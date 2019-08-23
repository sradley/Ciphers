use ciphers::{Cipher, Porta};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let porta = Porta::new("FORTIFICATION");

    let ctext = porta.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let porta = Porta::new("FORTIFICATION");

    let ptext = porta.decipher("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let porta = Porta::new("ZYXWVUTSRQPON");

    let ctext = porta.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let porta = Porta::new("ZYXWVUTSRQPON");

    let ptext = porta.decipher("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
