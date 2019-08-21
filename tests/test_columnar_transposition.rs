use ciphers::columnar_transposition;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("GERMAN");

    let ciphertext = columnar_transposition::cipher(plaintext, key);
    assert_eq!(ciphertext, "NALCEHWTTDTTFSEELEEDSOAFEAHL")
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("NALCEHWTTDTTFSEELEEDSOAFEAHL");
    let key = String::from("GERMAN");

    let plaintext = columnar_transposition::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPON");

    let ciphertext = columnar_transposition::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN"
    )
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("MZMZLYLYKXKXJWJWIVIVHUHUGTGTFSFSERERDQDQCPCPBOBOANAN");
    let key = String::from("ZYXWVUTSRQPON");

    let plaintext = columnar_transposition::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
