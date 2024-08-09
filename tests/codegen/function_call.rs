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

#[inline(never)]
fn foo(x: u32, y: u32) -> i32 {
    (x + y) as i32
}

/// CHECK-LABEL: main
/// CHECK: {{.*foo.*}}
#[no_mangle]
pub fn main() -> i32 {
    foo(0, 0)
}
