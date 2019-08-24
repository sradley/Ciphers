use ciphers::{Cipher, RailFence};

/// `encipher_small` test function.
#[test]
fn encipher_small() {
    let rail_fence = RailFence::new(4);

    let ctext = rail_fence.encipher("DEFENDTHEEASTWALLOFTHECASTLE");
    assert_eq!(ctext.unwrap(), "DTTFSEDHSWOTATFNEAALHCLEELEE");
}

/// `decipher_small` test function.
#[test]
fn decipher_small() {
    let rail_fence = RailFence::new(4);

    let ptext = rail_fence.decipher("DTTFSEDHSWOTATFNEAALHCLEELEE");
    assert_eq!(ptext.unwrap(), "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function.
#[test]
fn encipher_large() {
    let rail_fence = RailFence::new(6);

    let ctext = rail_fence.encipher("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(
        ctext.unwrap(),
        "AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT"
    );
}

/// `decipher_large` test function.
#[test]
fn decipher_large() {
    let rail_fence = RailFence::new(6);

    let ptext = rail_fence.decipher("AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT");
    assert_eq!(
        ptext.unwrap(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}

/// `encipher_lowercase` test function.
#[test]
fn encipher_lowercase() {
    let rail_fence = RailFence::new(4);

    let ctext = rail_fence.encipher("defendtheeastwallofthecastle");
    assert_eq!(ctext.unwrap(), "dttfsedhswotatfneaalhcleelee");
}

/// `decipher_lowercase` test function.
#[test]
fn decipher_lowercase() {
    let rail_fence = RailFence::new(4);

    let ptext = rail_fence.decipher("dttfsedhswotatfneaalhcleelee");
    assert_eq!(ptext.unwrap(), "defendtheeastwallofthecastle");
}
