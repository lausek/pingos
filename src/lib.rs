#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;

mod graphics;
mod vga;

use vga::Writer;

#[repr(C)]
struct gdt_entry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[no_mangle]
pub extern fn kmain() -> ! {
    
    use core::fmt::Write;

    let mut writer = Writer::new();
    let mut i = 0;
    
    let tokens = ["Hello"];

    loop {
        i += 1;
        write!(writer, "Hello World x{}\n", i).ok();
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
