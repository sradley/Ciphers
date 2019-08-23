use ciphers::{Cipher, ADFGVX};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");

    let ctext = adfgvx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext,
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let adfgvx = ADFGVX::new("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8", "GERMAN");

    let ptext = adfgvx.decipher("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let adfgvx = ADFGVX::new("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM", "ABCDEFGHIJKLMN");

    let ctext = adfgvx.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
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
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let adfgvx = ADFGVX::new("ph0qg64mea1yl2nofdxkr3cvs5zw7bj9uti8", "german");

    let ctext = adfgvx.encipher("defendtheeastwallofthecastle");
    assert_eq!(
        ctext,
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let adfgvx = ADFGVX::new("ph0qg64mea1yl2nofdxkr3cvs5zw7bj9uti8", "german");

    let ptext = adfgvx.decipher("ffdvdfadfxfgfgavfaffdxdxffdvdffdggagvgvxfaggdgadfadvfxgx");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}
