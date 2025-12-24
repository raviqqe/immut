#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
extern crate std;

/// Adds two numbers.
pub fn add(x: usize, y: usize) -> usize {
    x + y
}
