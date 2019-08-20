use ciphers::playfair;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let ciphertext = playfair::cipher(plaintext, key);
    assert_eq!(ciphertext, "RKPAWRPMYSELZCLFXUZFRSNQBPSA");
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("RKPAWRPMYSELZCLFXUZFRSNQBPSA");
    let key = String::from("ZGPTFOIHMUWDRCNYKEQAXVSBL");

    let plaintext = playfair::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEXASTWALLOFTHECASTLE");
}
