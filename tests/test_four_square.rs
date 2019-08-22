use ciphers::four_square::FourSquare;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("ATTACKATDAWN");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
    let four_square = FourSquare::new(key1, key2);

    let ctext = four_square.encipher(ptext);
    assert_eq!(ctext, "TIYBFHTIZBSY");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("TIYBFHTIZBSY");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");
    let four_square = FourSquare::new(key1, key2);

    let ptext = four_square.decipher(ctext);
    assert_eq!(ptext, "ATTACKATDAWN");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key1 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXY");
    let key2 = String::from("ZYXWVUTSRQPONMLKJIHGFEDCB");
    let four_square = FourSquare::new(key1, key2);

    let ctext = four_square.encipher(ptext);
    assert_eq!(
        ctext,
        "BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC");
    let key1 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXY");
    let key2 = String::from("ZYXWVUTSRQPONMLKJIHGFEDCB");
    let four_square = FourSquare::new(key1, key2);

    let ptext = four_square.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
