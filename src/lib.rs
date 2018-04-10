#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate multiboot2;
extern crate spin;

mod graphics;
mod vga;

use spin::Mutex;
use core::fmt::Write;
use vga::Writer;

static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

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

    let boot_info = unsafe { multiboot2::load(multiboot_adr) };
    let memory_map = boot_info.memory_map_tag().expect("No memory map tag");

    for area in memory_map.memory_areas() {
        write!(WRITER.lock(), "start 0x{} length: 0x{}\n", area.base_addr, area.length).ok();
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
    file: &'static str, line: u32, column: u32) -> ! {
    
    write!(WRITER.lock(), "Panic in {} on {}:{}", file, line, column).ok();

    loop {}
}
