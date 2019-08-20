use ciphers::running_key;

/// `cipher_small` test function ...
#[test]
fn cipher_small() {
    let plaintext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ciphertext = running_key::cipher(plaintext, key);
    assert_eq!(ciphertext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ciphertext = String::from("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let plaintext = running_key::decipher(ciphertext, key);
    assert_eq!(plaintext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `cipher_large` test function ...
#[test]
fn cipher_large() {
    let plaintext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let ciphertext = running_key::cipher(plaintext, key);
    assert_eq!(
        ciphertext,
        "ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ciphertext = String::from("ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF");
    let key = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let plaintext = running_key::decipher(ciphertext, key);
    assert_eq!(
        plaintext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
