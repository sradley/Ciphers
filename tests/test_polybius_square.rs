use ciphers::polybius_square;

/// `cipher_abcde_small` test function ...
#[test]
fn cipher_abcde_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher_abcde_small` test function ...
#[test]
fn decipher_abcde_small() {
    let ciphertext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_zebra_small` test function ...
#[test]
fn cipher_zebra_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB"
    );
}

/// `decipher_zebra_small` test function ...
#[test]
fn decipher_zebra_small() {
    let ciphertext = String::from("BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_abcde_large` test function ...
#[test]
fn cipher_abcde_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ABCDE");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAA"
    );
}

/// `decipher_abcde_large` test function ...
#[test]
fn decipher_abcde_large() {
    let ciphertext = String::from("EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAA");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ABCDE");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(
        plaintext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `cipher_zebra_large` test function ...
#[test]
fn cipher_zebra_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ZEBRA");

    let ciphertext = polybius_square::cipher(plaintext, key, chars);
    assert_eq!(
        ciphertext,
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZ"
    );
}

/// `decipher_zebra_large` test function ...
#[test]
fn decipher_zebra_large() {
    let ciphertext = String::from("AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ZEBRA");

    let plaintext = polybius_square::decipher(ciphertext, key, chars);
    assert_eq!(
        plaintext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
