use ciphers::adfgx;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    let keyword = String::from("GERMAN");

    let ciphertext = adfgx::cipher(plaintext, key, keyword);
    assert_eq!(ciphertext, "FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("FFDGDDADXDAFAFXAAFAFDXDXXFDGDAGDDXXFAFADAFDXDDXDDADGXXGX");
    let key = String::from("PHQGMEAYNOFDXKRCVSZWBUTIL");
    let keyword = String::from("GERMAN");

    let plaintext = adfgx::decipher(ciphertext, key, keyword);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
    let key = String::from("GEHDOSWILFQRCKUXZMNBPATVY");
    let keyword = String::from("ABCDEFGHIJKLMN");

    let ciphertext = adfgx::cipher(plaintext, key, keyword);
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

    let plaintext = adfgx::decipher(ciphertext, key, keyword);
    assert_eq!(plaintext, "ABCDEFGHIIKLMNOPQRSTUVWXYZABCDEFGHIIKLMNOPQRSTUVWXYZ");
}