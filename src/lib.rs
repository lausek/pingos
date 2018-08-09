#![feature(lang_items)]
#![feature(panic_implementation)]
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
#[macro_use]
mod graphics;
mod interrupt;

use core::{fmt::Write, panic::PanicInfo};
use graphics::vga::macros;

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
    
    vga_println!("starting...");

    memory::init(boot_info);

    interrupt::init();
    
    loop {}
}

#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(pi: &PanicInfo) -> ! {
    vga_println!("Panic occurred: {}", pi);
    loop {}
}
