use ciphers::{Cipher, FourSquare};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");

    let ctext = four_square.encipher("ATTACKATDAWN");
    assert_eq!(ctext.unwrap(), "TIYBFHTIZBSY");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");

    let ptext = four_square.decipher("TIYBFHTIZBSY");
    assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let four_square = FourSquare::new("ABCDEFGHIJKLMNOPQRSTUVWXY", "ZYXWVUTSRQPONMLKJIHGFEDCB");

    let ctext = four_square.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let four_square = FourSquare::new("ABCDEFGHIJKLMNOPQRSTUVWXY", "ZYXWVUTSRQPONMLKJIHGFEDCB");

    let ptext = four_square.decipher("BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let four_square = FourSquare::new("zgptfoihmuwdrcnykeqaxvsbl", "mfnbdcrhsaxyogvituewlqzkp");

    let ctext = four_square.encipher("attackatdawn");
    assert_eq!(ctext.unwrap(), "TIYBFHTIZBSY");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let four_square = FourSquare::new("zgptfoihmuwdrcnykeqaxvsbl", "mfnbdcrhsaxyogvituewlqzkp");

    let ptext = four_square.decipher("tiybfhtizbsy");
    assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
}
