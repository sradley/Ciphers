use ciphers::{Cipher, Playfair};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", 'X');

    let ctext = playfair.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", 'X');

    let ptext = playfair.decipher("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    assert_eq!(ptext.unwrap(), "DEFENDTHEXASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let playfair = Playfair::new("ZYXWVUTSRQPONMLKIHGFEDCBA", 'X');

    let ctext = playfair.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let playfair = Playfair::new("ZYXWVUTSRQPONMLKIHGFEDCBA", 'X');

    let ptext = playfair.decipher("EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIXKLMNOPQRSTUVWXYZABCDEFGHIXKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let playfair = Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'x');

    let ctext = playfair.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "rkpawrpmyselzclfxuzfrsnqbpsa");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let playfair = Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'x');

    let ptext = playfair.decipher("rkpawrpmyselzclfxuzfrsnqbpsa");
    assert_eq!(ptext.unwrap(), "defendthexastwallofthecastle");
}
