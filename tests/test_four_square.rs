use ciphers::{Cipher, CipherInputError, FourSquare};

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

/// `alphabet_not_25_chars` test function.
#[test]
#[should_panic]
fn alphabet_not_25_chars() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `key1_not_25_chars` test function.
#[test]
#[should_panic]
fn key1_not_25_chars() {
    FourSquare::new(
        "zgptfoihmuwdrcneqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `key2_not_25_chars` test function.
#[test]
#[should_panic]
fn key2_not_25_chars() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxgvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `alphabet_non_ascii` test function.
#[test]
#[should_panic]
fn alphabet_non_ascii() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdèfghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `alphabet_repeated_chars` test function.
#[test]
#[should_panic]
fn alphabet_repeated_chars() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiiklmnopqrstuvwxy",
        'x',
    );
}

/// `key1_repeated_chars` test function.
#[test]
#[should_panic]
fn key1_repeated_chars() {
    FourSquare::new(
        "zgppfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `key2_repeated_chars` test function.
#[test]
#[should_panic]
fn key2_repeated_chars() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcchsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `key1_not_in_alphabet` test function.
#[test]
#[should_panic]
fn key1_not_in_alphabet() {
    FourSquare::new(
        "zgptfojhmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `key2_not_in_alphabet` test function.
#[test]
#[should_panic]
fn key2_not_in_alphabet() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvjtuewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );
}

/// `pad_not_in_alphabet` test function.
#[test]
#[should_panic]
fn pad_not_in_alphabet() {
    FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'j',
    );
}

/// `ptext_not_in_alphabet` test function.
#[test]
fn ptext_not_in_alphabet() {
    let four_square = FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );

    let ctext = four_square.encipher("jttjckjtdawn");
    assert_eq!(ctext, Err(CipherInputError::NotInAlphabet));
}

/// `ctext_not_in_alphabet` test function.
#[test]
fn ctext_not_in_alphabet() {
    let four_square = FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );

    let ptext = four_square.decipher("tjybfhtjzbsy");
    assert_eq!(ptext, Err(CipherInputError::NotInAlphabet));
}

/// `ctext_uneven_chars` test function.
#[test]
fn ctext_uneven_chars() {
    let four_square = FourSquare::new(
        "zgptfoihmuwdrcnykeqaxvsbl",
        "mfnbdcrhsaxyogvituewlqzkp",
        "abcdefghiklmnopqrstuvwxyz",
        'x',
    );

    let ptext = four_square.decipher("tiybfhtizbs");
    assert_eq!(
        ptext,
        Err(CipherInputError::BadInput(String::from(
            "`ctext` must contain an even number of chars"
        )))
    );
}
