use ciphers::four_square;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("ATTACKATDAWN");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");

    let ciphertext = four_square::cipher(plaintext, key1, key2);
    assert_eq!(ciphertext, "TIYBFHTIZBSY");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("TIYBFHTIZBSY");
    let key1 = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");
    let key2 = String::from("MFNBDCRHSAXYOGVITUEWLQZKP");

    let plaintext = four_square::decipher(ciphertext, key1, key2);
    assert_eq!(plaintext, "ATTACKATDAWN");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key1 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXY");
    let key2 = String::from("ZYXWVUTSRQPONMLKJIHGFEDCB");

    let ciphertext = four_square::cipher(plaintext, key1, key2);
    assert_eq!(ciphertext, "BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC");
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("BZDXAQHTIRFLMOOMQKSIPBWEYCBZDXAQHTIRFLMOOMQKSIPBWEYC");
    let key1 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXY");
    let key2 = String::from("ZYXWVUTSRQPONMLKJIHGFEDCB");
    
    let plaintext = four_square::decipher(ciphertext, key1, key2);
    assert_eq!(plaintext, "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
}