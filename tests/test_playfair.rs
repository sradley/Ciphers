use ciphers::playfair;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let ciphertext = playfair::cipher(plaintext, key);
    assert_eq!(ciphertext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let plaintext = playfair::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEXASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");

    let ciphertext = playfair::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("EABCAKFGHYFPLMNOUQRSQZVWXYEABCAKFGHYFPLMNOUQRSQZVWXY");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");

    let plaintext = playfair::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIXKLMNOPQRSTUVWXYZABCDEFGHIXKLMNOPQRSTUVWXYZ"
    );
}
