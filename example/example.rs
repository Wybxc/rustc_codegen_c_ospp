#![feature(no_core)]
#![no_core]
#![no_main]

extern crate mini_core;

#[no_mangle]
pub fn main() -> i32 {
    0
}

#[no_mangle]
pub fn abc(a: u8) -> u8 {
    a * 2
}

#[no_mangle]
pub fn bcd(b: bool, a: u8) -> u8 {
    if b {
        a * 2
    } else {
        a * 3
    }
}
