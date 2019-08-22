use ciphers::running_key::RunningKey;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    let running_key = RunningKey::new(key);

    let ctext = running_key.encipher(ptext);
    assert_eq!(ctext, "KSBHBHLALIDMVGKYZKYAHXUAAWGM");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("KSBHBHLALIDMVGKYZKYAHXUAAWGM");
    let key = String::from("HOWDOESTHEDUCKKNOWTHATSAIDVICTOR");
    let running_key = RunningKey::new(key);

    let ptext = running_key.decipher(ctext);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");
    let running_key = RunningKey::new(key);

    let ctext = running_key.encipher(ptext);
    assert_eq!(
        ctext,
        "ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("ZOPRSUVXYABDEOQTVYADFVXACFZOPRSUVXYABDEOQTVYADFVXACF");
    let key = String::from("ZNNOOPPQQRRSSBCEFHIKLABDEGZNNOOPPQQRRSSBCEFHIKLABDEGA");
    let running_key = RunningKey::new(key);

    let ptext = running_key.decipher(ctext);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
