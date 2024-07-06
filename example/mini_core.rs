#![feature(no_core, lang_items)]
#![no_core]
#![no_main]
#![allow(internal_features)]

#[lang = "sized"]
pub trait Sized {}

#[no_mangle]
pub fn main() {}
