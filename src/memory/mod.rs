extern crate multiboot2;

use multiboot2::BootInformation;

pub mod alloc;
pub mod paging;

extern {
    static mut HEAP_BOTTOM: u8;
    static mut HEAP_TOP: u8;
}

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn from_addr(addr: usize) -> Frame {
        Frame {
            number: addr / PAGE_SIZE,
        } 
    }
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

pub fn init(boot_info: &BootInformation) {

    let memory_map = boot_info.memory_map_tag().expect("No memory map tag");
    let elf_sections = boot_info.elf_sections_tag().expect("No elf-sections");

    let (kstart, kend) = get_kernel_space(elf_sections.sections());
    let (mstart, mend) = (boot_info.start_address(), boot_info.total_size as usize);

    let mut alloc = alloc::fallocator::AreaFrameAllocator::new(kstart, kend, mstart, mend, memory_map.memory_areas()); 

}
