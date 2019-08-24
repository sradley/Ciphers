# Ciphers v0.1.0

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://meritbadge.herokuapp.com/ciphers)](https://crates.io/crates/ciphers)
[![Released API docs](https://docs.rs/ciphers/badge.svg)](https://docs.rs/ciphers)

Ciphers is a Rust library that provides implementations of many different
classical ciphers.

**Get started** by looking at the [documentation](https://docs.rs/ciphers).

## 1. Supported Ciphers
There are currently **16 different supported ciphers**.

| Transposition          | Monoalphabetic      | Polyalphabetic | Polygraphic | Other  |
| ---------------------- | ------------------- | -------------- | ----------- | ------ |
| Rail-fence             | Simple Substitution | Vigenere       | Playfair    | ADFGX  |
| Columnar Transposition | Caesar              | Beaufort       | Four-Square | ADFGVX |
|                        | Affine              | Autokey        |             |        |
|                        | Polybius Square     | Running Key    |             |        |
|                        | Atbash              | Porta          |             |        |

## 2. Installation
Simply put the following in your **Cargo.toml**.

```toml
[dependencies]
ciphers = "0.1.0"
```

## 3. Example Usage
E.g. using the **Vigenere** cipher.

```rust
use ciphers::{Cipher, Vigenere};

fn main() {
    let vigenere = Vigenere::new("examplekey");

    // encipher
    let ctext = vigenere.encipher("someexampletexthere").unwrap();
    println!("ciphertext: {}", ctext);

    // decipher
    let ptext = vigenere.decipher(&ctext).unwrap();
    println!("plaintext:  {}", ptext);
}
```

```sh
ciphertext: WLMQTIEWTJIQEJISIBI
plaintext:  SOMEEXAMPLETEXTHERE
```

## 4. To be Implemented
There are currently **7 different ciphers to be implemented**.

| Transposition | Monoalphabetic | Polyalphabetic | Polygraphic | Other                 |
| ------------- | -------------- | -------------- | ----------- | --------------------- |
|               | Rot13          | Gronsfeld      | Hill        | Bifid                 |
|               |                |                |             | Trifid                |
|               |                |                |             | Straddle Checkerboard |

## 5. Backlog
 * Refactor Four-Square to use custom alphabets.
 * Input validation.
 * The Columnar Transposition cipher can't deal with repeated letters in its
   keyword.

## 6. Random Notes
Alphabet-specific (but all-ascii) ciphers (remove uppercase validation):
 * Four-Square

## 7. Tests to Write
 * All the panic cases.
 * All the cases where the function returns an Err Result.