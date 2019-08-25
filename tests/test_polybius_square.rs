use ciphers::{Cipher, CipherInputError, PolybiusSquare};

/// `encipher_abcde_small` test function.
#[test]
fn encipher_abcde_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");

    let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext.unwrap(),
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher_abcde_small` test function.
#[test]
fn decipher_abcde_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");

    let ptext = ps.decipher("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_zebra_small` test function.
#[test]
fn encipher_zebra_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ZEBRA");

    let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext.unwrap(),
        "BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB"
    );
}

/// `decipher_zebra_small` test function.
#[test]
fn decipher_zebra_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ZEBRA");

    let ptext = ps.decipher("BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_abcde_large` test function.
#[test]
fn encipher_abcde_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ABCDE");

    let ctext = ps.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
}

/// `decipher_abcde_large` test function.
#[test]
fn decipher_abcde_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ABCDE");

    let ptext = ps.decipher(
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_zebra_large` test function.
#[test]
fn encipher_zebra_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ZEBRA");

    let ctext = ps.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
}

/// `decipher_zebra_large` test function.
#[test]
fn decipher_zebra_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ZEBRA");

    let ptext = ps.decipher(
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let ps = PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcde");

    let ctext = ps.encipher("defendtheeastwallofthecastle");
    assert_eq!(
        ctext.unwrap(),
        "cebccdbccbceebabbcbcbdeaebedbdcacacccdebabbcddbdeaebcabc"
    );
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let ps = PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcde");

    let ptext = ps.decipher("cebccdbccbceebabbcbcbdeaebedbdcacacccdebabbcddbdeaebcabc");
    assert_eq!(ptext.unwrap(), "defendtheeastwallofthecastle");
}

/// `key_non_ascii` test function.
#[test]
#[should_panic]
fn key_non_ascii() {
    PolybiusSquare::new("phqgiumèaylnofdxkrcvstzwb", "abcde");
}

/// `chars_non_ascii` test function.
#[test]
#[should_panic]
fn chars_non_ascii() {
    PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcdè");
}

/// `key_repeated_chars` test function.
#[test]
#[should_panic]
fn key_repeated_chars() {
    PolybiusSquare::new("phqgiomeaylnofdxkrcvstzwb", "abcde");
}

/// `chars_repeated_chars` test function.
#[test]
#[should_panic]
fn chars_repeated_chars() {
    PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "accde");
}

/// `key_wrong_length` test function.
#[test]
#[should_panic]
fn key_wrong_length() {
    PolybiusSquare::new("phqgiumeaylnofdxkrcvstzw", "abcde");
}

/// `chars_wrong_length` test function.
#[test]
#[should_panic]
fn chars_wrong_length() {
    PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcd");
}

/// `ptext_not_in_key` test function.
#[test]
fn ptext_not_in_key() {
    let ps = PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcde");

    let ctext = ps.encipher("djfendtheeastwallofthecasjle");
    assert_eq!(ctext, Err(CipherInputError::NotInAlphabet));
}
/// `ctext_not_in_chars` test function.
#[test]
fn ctext_not_in_chars() {
    let ps = PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcde");

    let ptext = ps.decipher("cebccdbccbceebabbcbcbdeazbedbdcacacccdebabbcddbdeaebcabc");
    assert_eq!(ptext, Err(CipherInputError::NotInAlphabet));
}

/// `ctext_uneven_chars` test function.
#[test]
fn ctext_uneven_chars() {
    let ps = PolybiusSquare::new("phqgiumeaylnofdxkrcvstzwb", "abcde");

    let ptext = ps.decipher("cebccdbccbceebabbcbcbdeaebedbdcacacccdebabbcddbdeaebcac");
    assert_eq!(
        ptext,
        Err(CipherInputError::BadInput(String::from(
            "`ctext` must contain an even number of chars"
        )))
    );
}
