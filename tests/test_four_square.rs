use ciphers::four_square;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("ATTACKATDAWN");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");

    let ciphertext = four_square::cipher(plaintext, key1, key2);
    assert_eq!(ciphertext, "TIYBFHTIZBSY");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("TIYBFHTIZBSY");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");

    let plaintext = four_square::decipher(ciphertext, key1, key2);
    assert_eq!(plaintext, "ATTACKATDAWN");
}
