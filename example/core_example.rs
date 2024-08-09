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
    extern "C" {
        pub fn puts(s: *const u8) -> i32;
        // pub fn printf(format: *const i8, ...) -> i32;
        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn memmove(dst: *mut u8, src: *const u8, size: usize);
        pub fn strncpy(dst: *mut u8, src: *const u8, size: usize);
    }
}

#[inline(never)]
fn foo(x: u32, y: u32) -> i32 {
    (x + y) as i32
}

#[inline(never)]
unsafe fn hello() {
    libc::puts(c"Hello, World!\n".as_ptr() as *const u8);
}

#[no_mangle]
pub fn main() -> i32 {
    unsafe { hello() };
    foo(0, 0)
}
