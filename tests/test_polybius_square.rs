use ciphers::polybius_square;

/// `cipher` test function ...
#[test]
fn cipher() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");

    let ciphertext = polybius_square::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher` test function ...
#[test]
fn decipher() {
    let ciphertext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");

    let plaintext = polybius_square::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}