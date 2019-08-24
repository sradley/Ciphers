//! # Input Handling
//! 
//! ...

use crate::{CipherResult, CipherInputError};

fn handle_alpha(input: &str) -> CipherResult {
    for c in input.chars() {
        if !c.is_ascii_alphabetic() {
            return Err(CipherInputError::NotAlphabetic)
        }
    }
    Ok(())
}

fn handle_alphanum(input: &str) -> CipherResult {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() {
            return Err(CipherInputError::NotAlphanumeric)
        }
    }
    Ok(())
}

fn handle_ascii(input: &str) -> CipherResult {
    for c in input.chars() {
        if !c.is_ascii() {
            return Err(CipherInputError::NotAscii)
        }
    }
    Ok(())
}

fn valid_alphabet() -> 