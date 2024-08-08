#![feature(no_core)]
#![no_core]
#![no_main]

extern crate mini_core;

#[no_mangle]
pub fn main() -> i32 {
    0
}

#[no_mangle]
pub fn foo(x: i32, y: i32) -> i8 {
    ((x + y) * (x - y) / 2) as i8
}
