#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;

mod graphics;
mod vga;

use vga::Writer;

#[no_mangle]
pub extern fn _start() -> ! {
    
    use core::fmt::Write;

    let mut writer = Writer::new();
    let mut i = 0;

    loop {
        i += 1;
        write!(writer, "Hello World x{}\n", i);
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
