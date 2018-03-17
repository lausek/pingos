const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
pub struct Buffer<'a, T: 'a> {
    pub chars: &'a mut [[T; 80]; 25],
    pub height: usize,
    pub width: usize,
}

impl <'a, T> Buffer<'a, T> {
    
    pub fn new(width: usize, height: usize) -> Buffer<'a, T> {
        Buffer {
            chars: unsafe { &mut *(0xb8000 as *mut [[T; BUFFER_WIDTH]; BUFFER_HEIGHT]) },
            height: BUFFER_HEIGHT,
            width: BUFFER_WIDTH,
        } 
    }

}
