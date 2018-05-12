pub mod allocator;

pub const PAGE_SIZE: usize = 4096;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

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
