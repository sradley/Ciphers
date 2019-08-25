use ciphers::{Cipher, CipherInputError, Playfair};

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

/// `key_not_25_chars` test function.
#[test]
#[should_panic]
fn key_not_25_chars() {
    Playfair::new("zgptfoihmuwdrcnykeqaxvsblj", 'x');
}

/// `key_non_ascii` test function.
#[test]
#[should_panic]
fn key_non_ascii() {
    Playfair::new("zgptfoihmuwdrcnykèqaxvsbl", 'x');
}

/// `key_repeated_chars` test function.
#[test]
#[should_panic]
fn key_repeated_chars() {
    Playfair::new("zgppfoihmuwdrcnykeqaxvsbl", 'x');
}

/// `pad_not_in_key` test function.
#[test]
#[should_panic]
fn pad_not_in_key() {
    Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'j');
}

/// `ptext_not_in_key` test function.
#[test]
fn ptext_not_in_key() {
    let playfair = Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'x');

    let ctext = playfair.encipher("defendtheeastwaljofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotInAlphabet));
}

/// `ctext_not_in_key` test function.
#[test]
fn ctext_not_in_key() {
    let playfair = Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'x');

    let ptext = playfair.decipher("rkpawrpmyselzcjfxuzfrsnqbpsa");
    assert_eq!(ptext, Err(CipherInputError::NotInAlphabet));
}

/// `ctext_uneven_chars` test function.
#[test]
fn ctext_uneven_chars() {
    let playfair = Playfair::new("zgptfoihmuwdrcnykeqaxvsbl", 'x');

    let ptext = playfair.decipher("rkpawrpmyselzcfxuzfrsnqbpsa");
    assert_eq!(
        ptext,
        Err(CipherInputError::BadInput(String::from(
            "`ctext` must contain an even number of chars"
        )))
    );
}
