# Ciphers
Ciphers is a Rust library that provides implementations of several different
classical ciphers.

Currently supported ciphers:
 * Affine
 * Atbash
 * Autokey
 * Beaufort
 * Caesar
 * Columnar Transposition
 * Polybius Square
 * Porta
 * Rail-fence
 * Running Key
 * Simple Substitution
 * Vigenere
 * Four-Square
 * Playfair
 * ADFGX
 * ADFGVX

## 1. Installation
...

## 2. Example Usage

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

## 3. To be Implemented
 * Rot13
 * Gronsfeld
 * Bifid
 * Trifid
 * Transposition
 * Straddle Checkerboard
 * Hill

## 4. Current Issues

 * Very little in the way of input validation.
 * Can't handle lowercase characters or non-alpha characters.
 * The Columnar Transposition cipher can't deal with repeated letters in its
   keyword.