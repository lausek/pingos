#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;

mod graphics;
mod vga;

use vga::Writer;

static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern fn _start() -> ! {

    let mut writer = Writer::new();
    
    loop {
        writer.write_str("Hello World\n");
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
