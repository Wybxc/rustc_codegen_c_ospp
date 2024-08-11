#![allow(internal_features)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::ffi::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

pub mod libc {
    use core::ffi::*;

    #[link(name = "c")]
    extern "C" {
        pub fn open(path: *const c_char, flags: c_int, mode: c_int) -> c_int;
        pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
        pub fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
        pub fn close(fd: c_int) -> c_int;
    }

    pub const O_RDONLY: c_int = 0;
    pub const O_WRONLY: c_int = 1;
    pub const O_RDWR: c_int = 2;
    pub const O_CREAT: c_int = 0o100;
}

fn eprintln(message: &[u8]) {
    unsafe { libc::write(STDERR_FILENO, message.as_ptr() as *const c_void, message.len()) };
}

const BUF_SIZE: usize = 4096;
const PERMISSIONS: i32 = 0o644;
const STDIN_FILENO: i32 = 0;
const STDOUT_FILENO: i32 = 1;
const STDERR_FILENO: i32 = 2;

#[no_mangle]
#[export_name = "main"] // <- this is needed to correctly print the signature of main
pub extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    if argc < 2 {
        eprintln(b"Usage: tee <file>\n");
        return 1;
    }

    let filename = unsafe { *argv.offset(1) };
    let fd = unsafe { libc::open(filename, libc::O_RDWR | libc::O_CREAT, PERMISSIONS) };
    if fd < 0 {
        eprintln(b"Failed to open file.\n");
        return 1;
    }

    let mut buf = [0u8; BUF_SIZE];
    loop {
        let n = unsafe { libc::read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };
        if n < 0 {
            eprintln(b"Failed to read from stdin.\n");
            return 1;
        }
        if n == 0 {
            break;
        }
        unsafe { libc::write(fd, buf.as_ptr() as *const c_void, n as usize) };
        unsafe { libc::write(STDOUT_FILENO, buf.as_ptr() as *const c_void, n as usize) };
    }

    unsafe { libc::close(fd) };
    0
}
