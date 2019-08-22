use ciphers::polybius_square::PolybiusSquare;
use ciphers::Cipher;

/// `encipher_abcde_small` test function ...
#[test]
fn encipher_abcde_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");
    let ps = PolybiusSquare::new(key, chars);

    let ctext = ps.encipher(ptext);
    assert_eq!(
        ctext,
        "CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC"
    );
}

/// `decipher_abcde_small` test function ...
#[test]
fn decipher_abcde_small() {
    let ctext = String::from("CEBCCDBCCBCEEBABBCBCBDEAEBEDBDCACACCCDEBABBCDDBDEAEBCABC");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ABCDE");
    let ps = PolybiusSquare::new(key, chars);

    let ptext = ps.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_zebra_small` test function ...
#[test]
fn encipher_zebra_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");
    let ps = PolybiusSquare::new(key, chars);

    let ctext = ps.encipher(ptext);
    assert_eq!(
        ctext,
        "BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB"
    );
}

/// `decipher_zebra_small` test function ...
#[test]
fn decipher_zebra_small() {
    let ctext = String::from("BAEBBREBBEBAAEZEEBEBERAZAEARERBZBZBBBRAEZEEBRRERAZAEBZEB");
    let key = String::from("PHQGIUMEAYLNOFDXKRCVSTZWB");
    let chars = String::from("ZEBRA");
    let ps = PolybiusSquare::new(key, chars);

    let ptext = ps.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_abcde_large` test function ...
#[test]
fn encipher_abcde_large() {
    let ptext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ABCDE");
    let ps = PolybiusSquare::new(key, chars);

    let ctext = ps.encipher(ptext);
    assert_eq!(
        ctext,
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
}

/// `decipher_abcde_large` test function ...
#[test]
fn decipher_abcde_large() {
    let ctext = String::from(
        "EEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBCBBBAAEADACABAAEEEDECEBEADEDDDCDBDBDACECDCCCBCABEBDBC\
        BBBAAEADACABAA"
    );
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ABCDE");
    let ps = PolybiusSquare::new(key, chars);

    let ptext = ps.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_zebra_large` test function ...
#[test]
fn encipher_zebra_large() {
    let ptext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ZEBRA");
    let ps = PolybiusSquare::new(key, chars);

    let ctext = ps.encipher(ptext);
    assert_eq!(
        ctext,
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
}

/// `decipher_zebra_large` test function ...
#[test]
fn decipher_zebra_large() {
    let ctext = String::from(
        "AAARABAEAZRARRRBRERERZBABRBBBEBZEAEREBEEEZZAZRZBZEZZAAARABAEAZRARRRBRERERZBABRBBBEBZEAEREB\
        EEEZZAZRZBZEZZ"
    );
    let key = String::from("ZYXWVUTSRQPONMLKIHGFEDCBA");
    let chars = String::from("ZEBRA");
    let ps = PolybiusSquare::new(key, chars);

    let ptext = ps.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
