#![allow(internal_features)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::ops::Mul;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

pub mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn puts(s: *const i8) -> i32;
    }
}

fn put_str(s: &str) {
    unsafe { libc::puts(s.as_ptr() as *const i8) };
}

fn put_i32(mut i: i32) {
    if i < 0 {
        put_str("-");
        i = -i;
    }

    if i >= 10 {
        put_i32(i / 10);
        i %= 10;
    }
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    put_str(&DIGITS[i as usize]);
}

#[derive(Copy, Clone)]
struct Complex {
    real: i32,
    imag: i32,
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + self.real * other.imag,
        }
    }
}

unsafe fn hello() {
    put_str("Hello, world!\n");
}

#[no_mangle]
pub fn main() -> i32 {
    unsafe { hello() };
    let i = Complex { real: 0, imag: 1 };
    let x = i * i * i * i;
    put_str("Result: ");
    put_i32(x.real);
    put_str("+");
    put_i32(x.imag);
    put_str("i\n");
    0
}
