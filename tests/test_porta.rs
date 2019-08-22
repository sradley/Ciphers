use ciphers::porta::Porta;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("FORTIFICATION");
    let porta = Porta::new(key);

    let ctext = porta.encipher(ptext);
    assert_eq!(ctext, "SYNNJSCVRNRLAHUTUKUCVRYRLANY");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("SYNNJSCVRNRLAHUTUKUCVRYRLANY");
    let key = String::from("FORTIFICATION");
    let porta = Porta::new(key);

    let ptext = porta.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");
    let porta = Porta::new(key);

    let ctext = porta.encipher(ptext);
    assert_eq!(
        ctext,
        "ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEG");
    let key = String::from("ZYXWVUTSRQPON");
    let porta = Porta::new(key);

    let ptext = porta.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
