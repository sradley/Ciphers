use ciphers::rail_fence::RailFence;
use ciphers::Cipher;

/// `encipher_small` test function ...
#[test]
fn encipher_small() {
    let ptext = String::from("DEFENDTHEEASTWALLOFTHECASTLE");
    let rail_fence = RailFence::new(4);

    let ctext = rail_fence.encipher(ptext, 4);
    assert_eq!(ctext, "DTTFSEDHSWOTATFNEAALHCLEELEE");
}

/// `decipher_small` test function ...
#[test]
fn decipher_small() {
    let ctext = String::from("DTTFSEDHSWOTATFNEAALHCLEELEE");
    let rail_fence = RailFence::new(4);

    let ptext = rail_fence.decipher(ctext, 4);
    assert_eq!(ptext, "DEFENDTHEEASTWALLOFTHECASTLE");
}

/// `encipher_large` test function ...
#[test]
fn encipher_large() {
    let ptext = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let rail_fence = RailFence::new(6);

    let ctext = rail_fence.encipher(ptext, 6);
    assert_eq!(
        ctext,
        "AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT"
    );
}

/// `decipher_large` test function ...
#[test]
fn decipher_large() {
    let ctext = String::from("AKUEOYBJLTVDFNPXZCIMSWCGMQWDHNRXBHLRVEGOQYAIKSUFPZJT");
    let rail_fence = RailFence::new(6);

    let ptext = rail_fence.decipher(ctext, 6);
    assert_eq!(
        ptext,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
