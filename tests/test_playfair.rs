use ciphers::playfair::Playfair;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let playfair = Playfair::new(key);

    let ctext = playfair.encipher(ptext);
    assert_eq!(ctext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let playfair = Playfair::new(key);

    let ptext = playfair.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEXASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let playfair = Playfair::new(key);

    let ctext = playfair.encipher(ptext);
    assert_eq!(
        ctext,
        "EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let playfair = Playfair::new(key);

    let ptext = playfair.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIXKLMNOPQRSTUVWXYZABCDEFGHIXKLMNOPQRSTUVWXYZ"
    );
}
