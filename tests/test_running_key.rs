use ciphers::{Cipher, CipherInputError, RunningKey};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ctext = running_key.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ptext = running_key.decipher("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let running_key = RunningKey::new("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let ctext = running_key.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let running_key = RunningKey::new("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let ptext = running_key.decipher("ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ctext = running_key.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ptext = running_key.decipher("ksbhbhlalidmvgkyzkyahxuaawgm");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `key_non_alpha` test function.
#[test]
#[should_panic]
fn key_non_alpha() {
    RunningKey::new("howdo3sth3duckknowthatsaidvictor");
}

/// `ptext_non_alpha` test function.
#[test]
fn ptext_non_alpha() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ctext = running_key.encipher("d3f3ndtheeastwallofthecastle");
    assert_eq!(ctext, Err(CipherInputError::NotAlphabetic));
}

/// `ctext_non_alpha` test function.
#[test]
fn ctext_non_alpha() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ptext = running_key.decipher("ksbhbhlalidmvgkyzkyahxu44wgm");
    assert_eq!(ptext, Err(CipherInputError::NotAlphabetic));
}

/// `ptext_longer_than_key` test function.
#[test]
fn ptext_longer_than_key() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ctext = running_key.encipher("defendtheeastwallofthecastleeeeeeeeeeee");
    assert_eq!(
        ctext,
        Err(CipherInputError::BadInput(String::from(
            "`ptext` cannot be longer than the key"
        )))
    );
}

/// `ctext_longer_than_key` test function.
#[test]
fn ctext_longer_than_key() {
    let running_key = RunningKey::new("howdoestheduckknowthatsaidvictor");

    let ptext = running_key.decipher("ksbhbhlalidmvgkyzkyahxuaawgmiuheiwquerpo");
    assert_eq!(
        ptext,
        Err(CipherInputError::BadInput(String::from(
            "`ctext` cannot be longer than the key"
        )))
    );
}
