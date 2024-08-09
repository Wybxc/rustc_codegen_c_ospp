#![feature(no_core)]
#![no_core]
#![no_main]

extern crate mini_core;

// CHECK-LABEL: test_cond
// CHECK: ==
// CHECK-NEXT: if
#[no_mangle]
pub fn test_cond(x: i32, y: i32, z: i32) -> i32 {
    if x == y {
        x + y + z
    } else {
        x - y - z
    }
}

#[no_mangle]
pub fn main() -> i32 {
    0
}
