use ciphers::adfgvx::ADFGVX;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    let keyword = String::from("GERMAN");
    let adfgvx = ADFGVX::new(key, keyword);

    let ctext = adfgvx.encipher(ptext);
    assert_eq!(
        ctext,
        "FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX"
    );
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("FFDVDFADFXFGFGAVFAFFDXDXFFDVDFFDGGAGVGVXFAGGDGADFADVFXGX");
    let key = String::from("PH0QG64MEA1YL2NOFDXKR3CVS5ZW7BJ9UTI8");
    let keyword = String::from("GERMAN");
    let adfgvx = ADFGVX::new(key, keyword);

    let ptext = adfgvx.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM");
    let keyword = String::from("ABCDEFGHIJKLMN");
    let adfgvx = ADFGVX::new(key, keyword);

    let ctext = adfgvx.encipher(ptext);
    assert_eq!(
        ctext,
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from(
        "DXDXAXDFFFDAGGVFFXFDXAXFGGVADAVDAXDFDGVFGGVFGAFAXAXFAXGDAVDXXDDGVFAAGGAFAFDGAXGDXDXXXDFFDA\
        AAGFXFDFDGGGVA"
    );
    let key = String::from("KNGC3FWOAEQ1ZYXBP5LT0U2684SJ97VDHIRM");
    let keyword = String::from("ABCDEFGHIJKLMN");
    let adfgvx = ADFGVX::new(key, keyword);

    let ptext = adfgvx.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
