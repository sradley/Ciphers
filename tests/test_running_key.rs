use ciphers::running_key::RunningKey;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ctext = running_key.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let running_key = RunningKey::new("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");

    let ptext = running_key.decipher("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let running_key = RunningKey::new("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let ctext = running_key.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext,
        "ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let running_key = RunningKey::new("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");

    let ptext = running_key.decipher("ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF");
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
