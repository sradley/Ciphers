use ciphers::{Cipher, FourSquare};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let four_square = FourSquare::new(
        "ZGPTFOIHMUWDRCNYKEQAXVSBL",
        "MFNBDCRHSAXYOGVITUEWLQZKP",
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        'X',
    );

    let ctext = four_square.encipher("ATTACKATDAWN");
    assert_eq!(ctext.unwrap(), "TIYBFHTIZBSY");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let four_square = FourSquare::new(
        "ZGPTFOIHMUWDRCNYKEQAXVSBL",
        "MFNBDCRHSAXYOGVITUEWLQZKP",
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        'X',
    );

    let ptext = four_square.decipher("TIYBFHTIZBSY");
    assert_eq!(ptext.unwrap(), "ATTACKATDAWN");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let four_square = FourSquare::new(
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        "ZYXWVUTSRQPONMLKIHGFEDCBA",
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        'X',
    );

    let ctext = four_square.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "BZDXAQHTIRFLNOPMRKTHQAXDZBBZDXAQHTIRFLNOPMRKTHQAXDZB"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let four_square = FourSquare::new(
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        "ZYXWVUTSRQPONMLKIHGFEDCBA",
        "ABCDEFGHIKLMNOPQRSTUVWXYZ",
        'X',
    );

    let ptext = four_square.decipher("BZDXAQHTIRFLNOPMRKTHQAXDZBBZDXAQHTIRFLNOPMRKTHQAXDZB");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let four_square = FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );

    let ctext = four_square.encipher("attackatdawn");
    assert_eq!(ctext.unwrap(), "tiybfhtizbsy");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let four_square = FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );

    let ptext = four_square.decipher("tiybfhtizbsy");
    assert_eq!(ptext.unwrap(), "attackatdawn");
}
