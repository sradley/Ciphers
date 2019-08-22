use ciphers::Cipher;
use ciphers::adfgx::ADFGX;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    let keyword = String::from("GERMAN");
    let adfgx = ADFGX::new(key, keyword);

    let ciphertext = adfgx.encipher(plaintext);
    assert_eq!(
        ciphertext,
        "FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX"
    );
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    let keyword = String::from("GERMAN");
    let adfgx = ADFGX::new(key, keyword);

    let plaintext = adfgx.decipher(ciphertext);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("GEHDOSWILFQRCKUXZMNBPATVY");
    let keyword = String::from("ABCDEFGHIJKLMN");
    let adfgx = ADFGX::new(key, keyword);

    let ciphertext = adfgx.encipher(plaintext);
    assert_eq!(
        ciphertext,
        "XAAXFDFGDFXGFFAAGDXDAFFXXFADGGDXFDFGADDGFFAADGADAFFXDGXGGDXXFFADDGAGFDGADAGXDGXXAAXXFFDFXG\
        AGFGDXDAGXXFAD"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from(
        "XAAXFDFGDFXGFFAAGDXDAFFXXFADGGDXFDFGADDGFFAADGADAFFXDGXGGDXXFFADDGAGFDGADAGXDGXXAAXXFFDFXG\
        AGFGDXDAGXXFAD"
    );
    let key = String::from("GEHDOSWILFQRCKUXZMNBPATVY");
    let keyword = String::from("ABCDEFGHIJKLMN");
    let adfgx = ADFGX::new(key, keyword);

    let plaintext = adfgx.decipher(ciphertext);
    assert_eq!(
        plaintext,
        "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ"
    );
}
