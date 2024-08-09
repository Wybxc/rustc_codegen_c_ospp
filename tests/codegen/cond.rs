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
