#![feature(no_core)]
#![no_core]
#![no_main]

extern crate mini_core;

// function signatures
// CHECK-LABEL: test_add
// CHECK-LABEL: test_sub
// CHECK-LABEL: test_mul

// CHECK-LABEL: test_add
// CHECK: +
// CHECK-NEXT: +
#[no_mangle]
pub fn test_add(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}

// CHECK-LABEL: test_sub
// CHECK: -
// CHECK-NEXT: -
#[no_mangle]
pub fn test_sub(x: i32, y: i32, z: i32) -> i32 {
    x - y - z
}

// CHECK-LABEL: test_mul
// CHECK: *
// CHECK-NEXT: *
#[no_mangle]
pub fn test_mul(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

#[no_mangle]
pub fn main() -> i32 {
    0
}
