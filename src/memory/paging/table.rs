use super::Entry;

const ENTRY_COUNT: usize = 512;

pub const P4: *mut Table = 0xffffffff_fffff000 as *mut _;

pub struct Table {
    entries: [Entry; ENTRY_COUNT], 
}
