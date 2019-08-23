use ciphers::polybius_square::PolybiusSquare;
use ciphers::Cipher;

/// `encipher_abcde_small` test function ...
#[test]
fn encipher_abcde_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");

    let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext,
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher_abcde_small` test function ...
#[test]
fn decipher_abcde_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ABCDE");

    let ptext = ps.decipher("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_zebra_small` test function ...
#[test]
fn encipher_zebra_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ZEBRA");

    let ctext = ps.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext,
        "BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB"
    );
}

/// `decipher_zebra_small` test function ...
#[test]
fn decipher_zebra_small() {
    let ps = PolybiusSquare::new("PHQGIUMEAYLNOFDXKRCVSTZWB", "ZEBRA");

    let ptext = ps.decipher("BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_abcde_large` test function ...
#[test]
fn encipher_abcde_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ABCDE");

    let ctext = ps.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
}

/// `decipher_abcde_large` test function ...
#[test]
fn decipher_abcde_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ABCDE");

    let ptext = ps.decipher(
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_zebra_large` test function ...
#[test]
fn encipher_zebra_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ZEBRA");

    let ctext = ps.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
}

/// `decipher_zebra_large` test function ...
#[test]
fn decipher_zebra_large() {
    let ps = PolybiusSquare::new("ZYXWVUTSRQPONMLKIHGFEDCBA", "ZEBRA");

    let ptext = ps.decipher(
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
