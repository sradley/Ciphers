use ciphers::playfair::Playfair;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let ctext = playfair.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let playfair = Playfair::new("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let ptext = playfair.decipher("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    assert_eq!(ptext, "DEFENDTHEXASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let playfair = Playfair::new("ZYXWVUTSRQPONMLKIHGFEDCBA");

    let ctext = playfair.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let playfair = Playfair::new("ZYXWVUTSRQPONMLKIHGFEDCBA");

    let ptext = playfair.decipher("EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY");
    assert_eq!(
        ptext,
        "ABCDEFGHIXKLMNOPQRSTUVWXYZABCDEFGHIXKLMNOPQRSTUVWXYZ"
    );
}
