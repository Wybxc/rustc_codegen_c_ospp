#![feature(no_core)]
#![no_core]
#![no_main]

extern crate mini_core;

// CHECK: +
// CHECK-NEXT: +
#[no_mangle]
pub fn foo(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}

#[no_mangle]
pub fn main() -> i32 {
    0
}
