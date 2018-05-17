use memory::Frame;
use multiboot2::{MemoryAreaIter, MemoryArea};

pub mod fallocator;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
