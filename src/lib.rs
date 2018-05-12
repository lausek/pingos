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
mod memory;

use spin::Mutex;
use core::fmt::Write;
use vga::Writer;

use memory::FrameAllocator;

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

fn get_kernel_space(sections: multiboot2::ElfSectionIter) -> (usize, usize) {
    let mut space: (usize, usize) = (0, 0);
    for s in sections {
        let bot = s.addr as usize;
        let top = (s.addr + s.size) as usize;

        if space.0 < bot {
            space.0 = bot;
        }
        if space.1 < top {
            space.1 = top;
        }
    }
    space
}

#[no_mangle]
pub extern fn kmain(multiboot_adr: usize) -> ! {

    let boot_info = unsafe { multiboot2::load(multiboot_adr) };
    let memory_map = boot_info.memory_map_tag().expect("No memory map tag");
    let elf_sections = boot_info.elf_sections_tag().expect("No elf-sections");

    for area in memory_map.memory_areas() {
        write!(WRITER.lock(), "start 0x{} length: 0x{}\n", area.base_addr, area.length).ok();
    }

    write!(WRITER.lock(), "kernel sections:\n");
    for section in elf_sections.sections() {
        write!(WRITER.lock(), "addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}\n", section.addr, section.size, section.flags).ok();
    } 

    let (kstart, kend) = get_kernel_space(elf_sections.sections());
    let (mstart, mend) = (boot_info.start_address(), boot_info.end_address());

    let mut alloc = memory::allocator::AreaFrameAllocator::new(kstart, kend, mstart, mend, memory_map.memory_areas()); 

    for i in 0.. {
        if let None = alloc.allocate_frame() {
            write!(WRITER.lock(), "allocated {} frames\n", i);
            break;
        }
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
