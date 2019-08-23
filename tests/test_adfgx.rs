use ciphers::adfgx::ADFGX;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let adfgx = ADFGX::new("PHQGMEAYNOFDXKRCVSZWBUTIL", "GERMAN");

    let ctext = adfgx.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(
        ctext,
        "FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX"
    );
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let adfgx = ADFGX::new("PHQGMEAYNOFDXKRCVSZWBUTIL", "GERMAN");

    let ptext = adfgx.decipher("FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let adfgx = ADFGX::new("GEHDOSWILFQRCKUXZMNBPATVY", "ABCDEFGHIJKLMN");

    let ctext = adfgx.encipher("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "XAAXFDFGDFXGFFAAGDXDAFFXXFADGGDXFDFGADDGFFAADGADAFFXDGXGGDXXFFADDGAGFDGADAGXDGXXAAXXFFDFXG\
        AGFGDXDAGXXFAD"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let adfgx = ADFGX::new("GEHDOSWILFQRCKUXZMNBPATVY", "ABCDEFGHIJKLMN");

    let ptext = adfgx.decipher(
        "XAAXFDFGDFXGFFAAGDXDAFFXXFADGGDXFDFGADDGFFAADGADAFFXDGXGGDXXFFADDGAGFDGADAGXDGXXAAXXFFDFXG\
        AGFGDXDAGXXFAD"
    );
    assert_eq!(
        ptext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
