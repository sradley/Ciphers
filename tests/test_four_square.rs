use ciphers::{Cipher, FourSquare};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");

    let ctext = four_square.encipher("ATTACKATDAWN");
    assert_eq!(ctext, "TIYBFHTIZBSY");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let four_square = FourSquare::new("ZGPTFOIHMUWDRCNYKEQAXVSBL", "MFNBDCRHSAXYOGVITUEWLQZKP");

    let ptext = four_square.decipher("TIYBFHTIZBSY");
    assert_eq!(ptext, "ATTACKATDAWN");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let four_square = FourSquare::new("ABCDEFGHIJKLMNOPQRSTUVWXY", "ZYXWVUTSRQPONMLKJIHGFEDCB");

    let ctext = four_square.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let four_square = FourSquare::new("ABCDEFGHIJKLMNOPQRSTUVWXY", "ZYXWVUTSRQPONMLKJIHGFEDCB");

    let ptext = four_square.decipher("BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC");
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
