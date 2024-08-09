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

// expect three int params
// CHECK-LABEL: foo
// CHECK: {{((int32_t .*,?\s?){3})}}
#[no_mangle]
pub fn foo(_x: i32, _y: i32, _z: i32) -> i32 {
    0
}

#[no_mangle]
pub fn main() -> i32 {
    0
}
