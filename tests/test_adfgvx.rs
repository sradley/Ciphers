use ciphers::{Cipher, ADFGVX};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");

    let ctext = adfgvx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext.unwrap(),
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");

    let ptext = adfgvx.decipher("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let adfgvx = ADFGVX::new("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM", "ABCDEFGHIJKLMN");

    let ctext = adfgvx.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let adfgvx = ADFGVX::new("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM", "ABCDEFGHIJKLMN");

    let ptext = adfgvx.decipher(
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let adfgvx = ADFGVX::new("ph0qg64mea1yl2nofdxkr3cvs5zw7bj9uti8", "german");

    let ctext = adfgvx.encipher("defendtheeastwallofthecastle");
    assert_eq!(
        ctext.unwrap(),
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let adfgvx = ADFGVX::new("ph0qg64mea1yl2nofdxkr3cvs5zw7bj9uti8", "german");

    let ptext = adfgvx.decipher("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    assert_eq!(ptext.unwrap(), "defendtheeastwallofthecastle");
}

/// `key_not_36_chars` test function.
#[test]
#[should_panic]
fn key_not_36_chars() {
    ADFGVX::new("ph0qg64mea1yl2nofdxkr3cvs5zw7bj9ut", "german");
}
