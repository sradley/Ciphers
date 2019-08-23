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
use ciphers::Cipher;
use ciphers::vigenere::Vigenere;

fn main() {
    let vigenere = Vigenere::new("EXAMPLEKEY");

    // encipher
    let ctext = vigenere.encipher("SOMEEXAMPLETEXTHERE");
    println!("ciphertext: {}", ctext);

    // decipher
    let ptext = vigenere.decipher(&ctext);
    println!("plaintext:  {}", ptext);
}
```

```sh
ciphertext: WLMQTIEWTJIQEJISIBI
plaintext:  SOMEEXAMPLETEXTHERE
```

## 4. To be Implemented
There are currently 6 different ciphers to be implemented.

| Transposition | Monoalphabetic | Polyalphabetic | Polygraphic | Other                 |
| ------------- | -------------- | -------------- | ----------- | --------------------- |
|               | Rot13          | Gronsfeld      | Hill        | Bifid                 |
|               |                |                |             | Trifid                |
|               |                |                |             | Straddle Checkerboard |

## 5. Known Issues
 * Very little in the way of input validation.
 * Can't handle lowercase characters or non-alpha characters.
 * The Columnar Transposition cipher can't deal with repeated letters in its
   keyword.