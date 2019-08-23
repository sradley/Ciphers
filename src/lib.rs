//! # Ciphers
//!
//! Ciphers is a Rust library that provides implementations of many different classical ciphers.

mod adfgvx;
mod adfgx;
mod affine;
mod atbash;
mod autokey;
mod beaufort;
mod caesar;
mod columnar_transposition;
mod four_square;
mod playfair;
mod polybius_square;
mod porta;
mod rail_fence;
mod running_key;
mod substitution;
mod vigenere;

// re-exports
pub use adfgvx::ADFGVX;
pub use adfgx::ADFGX;
pub use affine::Affine;
pub use atbash::Atbash;
pub use autokey::Autokey;
pub use beaufort::Beaufort;
pub use caesar::Caesar;
pub use columnar_transposition::ColumnarTransposition;
pub use four_square::FourSquare;
pub use playfair::Playfair;
pub use polybius_square::PolybiusSquare;
pub use porta::Porta;
pub use rail_fence::RailFence;
pub use running_key::RunningKey;
pub use substitution::Substitution;
pub use vigenere::Vigenere;

/// `Cipher` trait defines the implementation for cipher functionality.
pub trait Cipher {
    /// `encipher` method should take plaintext as a string reference, and return the ciphertext as
    /// a String object.
    fn encipher(&self, ptext: &str) -> String;

    /// `decipher` method should take the ciphertext as a string reference, and return the plaintext
    /// as a String object.
    fn decipher(&self, ctext: &str) -> String;
}

static TABULA_RECTA: [[u8; 26]; 26] = [
    [
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
        88, 89, 90,
    ],
    [
        66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
        89, 90, 65,
    ],
    [
        67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
        90, 65, 66,
    ],
    [
        68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
        65, 66, 67,
    ],
    [
        69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65,
        66, 67, 68,
    ],
    [
        70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66,
        67, 68, 69,
    ],
    [
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67,
        68, 69, 70,
    ],
    [
        72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68,
        69, 70, 71,
    ],
    [
        73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69,
        70, 71, 72,
    ],
    [
        74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70,
        71, 72, 73,
    ],
    [
        75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74,
    ],
    [
        76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75,
    ],
    [
        77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73,
        74, 75, 76,
    ],
    [
        78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
        75, 76, 77,
    ],
    [
        79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
        76, 77, 78,
    ],
    [
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76,
        77, 78, 79,
    ],
    [
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
        78, 79, 80,
    ],
    [
        82, 83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
        79, 80, 81,
    ],
    [
        83, 84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
        80, 81, 82,
    ],
    [
        84, 85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83,
    ],
    [
        85, 86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
        82, 83, 84,
    ],
    [
        86, 87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
        83, 84, 85,
    ],
    [
        87, 88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
        84, 85, 86,
    ],
    [
        88, 89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84,
        85, 86, 87,
    ],
    [
        89, 90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
        86, 87, 88,
    ],
    [
        90, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
        87, 88, 89,
    ],
];
