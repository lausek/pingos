use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kframes: (Frame, Frame), // kernel frames
    mframes: (Frame, Frame), // multiboot frames
}

impl AreaFrameAllocator {
    pub fn new(ks: usize, ke: usize, ms: usize, me: usize, iter: MemoryAreaIter)
        -> AreaFrameAllocator
        {
            let mut alloc = AreaFrameAllocator {
                next_free_frame: Frame::from_addr(0),
                current_area: None,
                areas: iter,
                kframes: (Frame::from_addr(ks), Frame::from_addr(ke)),
                mframes: (Frame::from_addr(ms), Frame::from_addr(me)),
            }; 
            alloc.next_area();
            alloc
        }

    fn next_area(&mut self) {
        self.current_area = self.areas.next();

        if let Some(area) = self.current_area {
            self.next_free_frame = Frame::from_addr(area.base_addr as usize);
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        match self.current_area {
            Some(area) => {
               
                let frame = Frame { number: self.next_free_frame.number };
                let last_frame = {
                    let addr = area.base_addr + area.length - 1; 
                    Frame::from_addr(addr as usize)
                };
                
                if last_frame < frame {
                    self.next_area();
                } else if self.kframes.0 <= frame && self.kframes.1 <= frame {
                    self.next_free_frame = Frame {
                        number: self.kframes.1.number + 1
                    };
                } else if self.mframes.0 <= frame && self.mframes.1 <= frame {
                    self.next_free_frame = Frame {
                        number: self.mframes.1.number + 1
                    };
                } else {
                    // frame is valid -> return
                    self.next_free_frame.number += 1;
                    return Some(frame);
                } 

                self.allocate_frame()
            },
            _ => None,
        }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!();
    }
}
