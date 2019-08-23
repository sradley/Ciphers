# Ciphers
Ciphers is a Rust library that provides implementations of several different
classical ciphers.

## Table of Contents
1. [Supported Ciphers]()
2. [Installation]()
3. [Example Usage]()
4. [To be Implemented]()
5. [Known Issues]()

## 1. Supported Ciphers
| Transposition          | Monoalphabetic      | Polyalphabetic | Polygraphic | Other  |
| ---------------------- | ------------------- | -------------- | ----------- | ------ |
| Rail-fence             | Simple Substitution | Vigenere       | Playfair    | ADFGX  |
| Columnar Transposition | Caesar              | Beaufort       | Four-Square | ADFGVX |
|                        | Affine              | Autokey        |             |        |
|                        | Polybius Square     | Running Key    |             |        |
|                        | Atbash              | Porta          |             |        |

## 2. Installation
...

## 3. Example Usage
E.g. using the Vigenere cipher.
```rust
use ciphers::Cipher;
use ciphers::vigenere::Vigenere;

let vigenere = Vigenere::new(String::from("EXAMPLEKEY"));
let ptext = String::from("SOMEXAMPLETEXTHERE");

// encipher
let ctext = vigenere.encipher(ptext);
println!("{}", ctext);

// decipher
let ptext = vigenere.decipher(ctext);
println!("{}", ptext);
```

## 4. To be Implemented
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