use ciphers::polybius_square;

/// `cipher_abcde` test function ...
#[test]
fn cipher_abcde() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher_abcde` test function ...
#[test]
fn decipher_abcde() {
    let ciphertext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_zebra` test function ...
#[test]
fn cipher_zebra() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB"
    );
}

/// `decipher_zebra` test function ...
#[test]
fn decipher_zebra() {
    let ciphertext = String::from("BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
