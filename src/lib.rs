#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

mod memory;
mod interrupt;
mod graphics;

#[macro_use]
mod vga;
#[macro_use]
mod log;

use core::fmt::Write;
use spin::Mutex;
use log::Logger;

lazy_static! {
    pub static ref SYSLOG: Mutex<Logger> = Mutex::new(Logger::new());
}

//#[repr(C)]
//struct gdt_entry {
//    limit_low: u16,
//    base_low: u16,
//    base_middle: u8,
//    access: u8,
//    granularity: u8,
//    base_high: u8,
//}

#[no_mangle]
pub extern fn kmain(mboot_addr: usize) -> ! {

    let boot_info = unsafe { multiboot2::load(mboot_addr) };
    
    log!(SYSLOG, "starting...");

    memory::init(boot_info);

    interrupt::init();

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
    file: &'static str, line: u32, column: u32) -> ! {
    log!(SYSLOG, "Panic in {} on {}:{}");//, file, line, column);
    loop {}
}
