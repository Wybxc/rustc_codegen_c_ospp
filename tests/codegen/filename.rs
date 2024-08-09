// CHECK: filename

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

// CHECK: function_name
#[no_mangle]
pub fn function_name() -> i32 {
    0
}

#[no_mangle]
pub fn main() -> i32 {
    0
}
