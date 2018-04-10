#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate multiboot2;

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
pub extern fn kmain(multiboot_adr: usize) -> ! {
    
    use core::fmt::Write;

    let boot_info = unsafe { multiboot2::load(multiboot_adr) };
    let memory_map = boot_info.memory_map_tag().expect("No memory map tag");
    let mut writer = Writer::new();

    for area in memory_map.memory_areas() {
        write!(writer, "start 0x{} length: 0x{}\n", area.base_addr, area.length).ok();
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
