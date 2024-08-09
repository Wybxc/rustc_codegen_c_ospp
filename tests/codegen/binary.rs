#![allow(internal_features)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

pub mod libc {
    #[link(name = "c")]
    extern "C" {}
}

// CHECK-LABEL: rustc_codegen_c: implementation

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
