use ciphers::adfgvx;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    let keyword = String::from("GERMAN");

    let ciphertext = adfgvx::cipher(plaintext, key, keyword);
    assert_eq!(
        ciphertext,
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    let keyword = String::from("GERMAN");

    let plaintext = adfgvx::decipher(ciphertext, key, keyword);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM");
    let keyword = String::from("ABCDEFGHIJKLMN");

    let ciphertext = adfgvx::cipher(plaintext, key, keyword);
    assert_eq!(
        ciphertext,
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from(
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
    let key = String::from("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM");
    let keyword = String::from("ABCDEFGHIJKLMN");

    let plaintext = adfgvx::decipher(ciphertext, key, keyword);
    assert_eq!(
        plaintext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
